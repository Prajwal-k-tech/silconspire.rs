use clap::Parser;
use rand::prelude::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the problem file
    #[arg(long, default_value = "silicon_spire.txt")]
    input_file: String,

    /// Number of wolves in the pack
    #[arg(long, default_value_t = 30)]
    pack_size: usize,

    /// Number of GWO iterations
    #[arg(long, default_value_t = 100)]
    max_iterations: usize,

    /// Number of iterations for the Tabu Search
    #[arg(long, default_value_t = 50)]
    ts_iterations: usize,

    /// The size of the tabu list
    #[arg(long, default_value_t = 10)]
    tabu_tenure: usize,
}

fn validate_args(args: &Args) -> Result<(), &'static str> {
    if args.pack_size < 3 {
        return Err("pack size must be at least 3 (needed for alpha/beta/delta)");
    }
    if args.max_iterations == 0 {
        return Err("max iterations must be positive");
    }
    if args.tabu_tenure == 0 {
        return Err("tabu tenure must be positive");
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct Problem {
    n: usize,
    distance: Vec<Vec<i32>>,
    flow: Vec<Vec<i32>>,
}

#[derive(Clone, Debug)]
struct Wolf {
    position: Vec<f64>,
    permutation: Vec<usize>,
    fitness: i64,
}

fn load_problem(filename: &str) -> Result<Problem, io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let mut line_idx = 0;

    // Read problem size
    let first_line = lines
        .first()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing problem size"))?;
    let n: usize = first_line.trim().parse().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid problem size: {}", e),
        )
    })?;
    if n == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Problem size must be positive",
        ));
    }
    line_idx += 1;

    // Skip empty line
    if line_idx < lines.len() && lines[line_idx].trim().is_empty() {
        line_idx += 1;
    }

    // Read distance matrix
    let mut distance = Vec::with_capacity(n);
    for _ in 0..n {
        if line_idx >= lines.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unexpected end of file reading distance matrix",
            ));
        }
        let row: Result<Vec<i32>, _> = lines[line_idx]
            .split_whitespace()
            .map(|x| x.parse())
            .collect();
        let row = row.map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid distance matrix data: {}", e),
            )
        })?;
        if row.len() != n {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Distance matrix row length mismatch",
            ));
        }
        distance.push(row);
        line_idx += 1;
    }

    // Skip empty line
    if line_idx < lines.len() && lines[line_idx].trim().is_empty() {
        line_idx += 1;
    }

    // Read flow matrix
    let mut flow = Vec::with_capacity(n);
    for _ in 0..n {
        if line_idx >= lines.len() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unexpected end of file reading flow matrix",
            ));
        }
        let row: Result<Vec<i32>, _> = lines[line_idx]
            .split_whitespace()
            .map(|x| x.parse())
            .collect();
        let row = row.map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid flow matrix data: {}", e),
            )
        })?;
        if row.len() != n {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Flow matrix row length mismatch",
            ));
        }
        flow.push(row);
        line_idx += 1;
    }

    if lines[line_idx..].iter().any(|line| !line.trim().is_empty()) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unexpected trailing data after flow matrix",
        ));
    }

    Ok(Problem { n, distance, flow })
}

fn calculate_cost(permutation: &[usize], problem: &Problem) -> i64 {
    let n = problem.n;
    let mut cost = 0i64;

    for i in 0..n {
        for j in 0..n {
            cost +=
                problem.flow[i][j] as i64 * problem.distance[permutation[i]][permutation[j]] as i64;
        }
    }

    cost
}

fn continuous_to_permutation(position: &[f64]) -> Vec<usize> {
    let mut indexed: Vec<(usize, f64)> = position
        .iter()
        .enumerate()
        .map(|(i, &val)| (i, val))
        .collect();

    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    indexed.iter().map(|(idx, _)| *idx).collect()
}

fn apply_tabu_search(
    wolf: &mut Wolf,
    problem: &Problem,
    ts_iterations: usize,
    tabu_tenure: usize,
    global_best: &mut i64,
) {
    let n = problem.n;
    let mut tabu_list: VecDeque<(usize, usize)> = VecDeque::new();
    let mut current_perm = wolf.permutation.clone();
    let mut current_fitness = wolf.fitness;
    let mut best_perm = current_perm.clone();
    let mut best_fitness = current_fitness;

    for _ in 0..ts_iterations {
        let mut best_move: Option<(usize, usize, Vec<usize>, i64)> = None;
        let mut best_move_fitness = i64::MAX;

        // Explore all 2-opt moves
        for i in 0..n {
            for j in (i + 1)..n {
                let mut new_perm = current_perm.clone();
                new_perm.swap(i, j);

                let move_tuple = if i < j { (i, j) } else { (j, i) };
                let fitness = calculate_cost(&new_perm, problem);

                let is_tabu = tabu_list.contains(&move_tuple);
                let aspiration = fitness < *global_best;

                // Accept move if not tabu or aspiration criterion is met
                if (!is_tabu || aspiration) && fitness < best_move_fitness {
                    best_move = Some((i, j, new_perm, fitness));
                    best_move_fitness = fitness;
                }
            }
        }

        // Apply the best move found
        if let Some((i, j, new_perm, fitness)) = best_move {
            let move_tuple = if i < j { (i, j) } else { (j, i) };

            // Add move to tabu list
            tabu_list.push_back(move_tuple);
            if tabu_list.len() > tabu_tenure {
                tabu_list.pop_front();
            }

            // Update current solution
            current_perm = new_perm;
            current_fitness = fitness;

            // Update best solution if improved
            if current_fitness < best_fitness {
                best_perm = current_perm.clone();
                best_fitness = current_fitness;
            }

            // Update global best if improved
            if current_fitness < *global_best {
                *global_best = current_fitness;
            }
        } else {
            // No valid move found, break
            break;
        }
    }

    // Update wolf with best solution found
    wolf.permutation = best_perm;
    wolf.fitness = best_fitness;
}

fn main() {
    let args = Args::parse();
    if let Err(message) = validate_args(&args) {
        eprintln!("error: {message}");
        std::process::exit(2);
    }

    // Load problem data
    let problem = match load_problem(&args.input_file) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error loading problem file '{}': {}", args.input_file, e);
            std::process::exit(1);
        }
    };

    println!(
        "Loaded QAP instance with {} facilities/locations",
        problem.n
    );
    println!(
        "Pack size: {}, Max iterations: {}, TS iterations: {}, Tabu tenure: {}",
        args.pack_size, args.max_iterations, args.ts_iterations, args.tabu_tenure
    );
    println!();

    let mut rng = rand::thread_rng();
    let n = problem.n;

    // Initialize wolf pack
    let mut pack: Vec<Wolf> = (0..args.pack_size)
        .map(|_| {
            let position: Vec<f64> = (0..n).map(|_| rng.gen()).collect();
            let permutation = continuous_to_permutation(&position);
            let fitness = calculate_cost(&permutation, &problem);
            Wolf {
                position,
                permutation,
                fitness,
            }
        })
        .collect();

    // Sort pack by fitness to identify Alpha, Beta, Delta
    pack.sort_by_key(|w| w.fitness);
    let mut global_best = pack[0].fitness;

    println!("Initial best fitness: {}", global_best);
    println!();

    // Main GWO loop
    for t in 0..args.max_iterations {
        let a = 2.0 - 2.0 * t as f64 / args.max_iterations as f64;

        // Get current Alpha, Beta, Delta
        let alpha = pack[0].clone();
        let beta = pack[1.min(args.pack_size - 1)].clone();
        let delta = pack[2.min(args.pack_size - 1)].clone();

        // Update positions of all wolves
        for wolf in pack.iter_mut() {
            for d in 0..n {
                // Alpha influence
                let r1: f64 = rng.gen();
                let r2: f64 = rng.gen();
                let a1 = 2.0 * a * r1 - a;
                let c1 = 2.0 * r2;
                let x_alpha = alpha.position[d];
                let d_alpha = (c1 * x_alpha - wolf.position[d]).abs();
                let x1 = x_alpha - a1 * d_alpha;

                // Beta influence
                let r1: f64 = rng.gen();
                let r2: f64 = rng.gen();
                let a2 = 2.0 * a * r1 - a;
                let c2 = 2.0 * r2;
                let x_beta = beta.position[d];
                let d_beta = (c2 * x_beta - wolf.position[d]).abs();
                let x2 = x_beta - a2 * d_beta;

                // Delta influence
                let r1: f64 = rng.gen();
                let r2: f64 = rng.gen();
                let a3 = 2.0 * a * r1 - a;
                let c3 = 2.0 * r2;
                let x_delta = delta.position[d];
                let d_delta = (c3 * x_delta - wolf.position[d]).abs();
                let x3 = x_delta - a3 * d_delta;

                // Update position
                wolf.position[d] = (x1 + x2 + x3) / 3.0;
            }

            // Convert new position to permutation and calculate fitness
            wolf.permutation = continuous_to_permutation(&wolf.position);
            wolf.fitness = calculate_cost(&wolf.permutation, &problem);
        }

        // Sort pack to get new Alpha
        pack.sort_by_key(|w| w.fitness);

        // Hybridization: Apply Tabu Search to Alpha wolf
        let mut alpha_improved = pack[0].clone();
        apply_tabu_search(
            &mut alpha_improved,
            &problem,
            args.ts_iterations,
            args.tabu_tenure,
            &mut global_best,
        );

        // Replace Alpha if improved
        pack[0] = alpha_improved;

        // Sort pack again after hybridization
        pack.sort_by_key(|w| w.fitness);

        // Update global best
        if pack[0].fitness < global_best {
            global_best = pack[0].fitness;
        }

        // Print progress
        println!("Iteration {}, Best Fitness: {}", t + 1, pack[0].fitness);
    }

    // Output final results
    println!();
    println!("Final Best Permutation: {:?}", pack[0].permutation);
    println!("Minimum Cost: {}", pack[0].fitness);
}

#[cfg(test)]
mod tests {
    use super::{validate_args, Args};

    fn valid_args() -> Args {
        Args {
            input_file: "silicon_spire.txt".to_string(),
            pack_size: 3,
            max_iterations: 1,
            ts_iterations: 0,
            tabu_tenure: 1,
        }
    }

    #[test]
    fn accepts_minimum_valid_values_and_disabled_tabu_search() {
        assert!(validate_args(&valid_args()).is_ok());
    }

    #[test]
    fn rejects_pack_smaller_than_three() {
        let mut args = valid_args();
        args.pack_size = 2;
        assert!(validate_args(&args).is_err());
    }

    #[test]
    fn rejects_zero_iterations() {
        let mut args = valid_args();
        args.max_iterations = 0;
        assert!(validate_args(&args).is_err());
    }

    #[test]
    fn rejects_zero_tabu_tenure() {
        let mut args = valid_args();
        args.tabu_tenure = 0;
        assert!(validate_args(&args).is_err());
    }
}
