use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn load_problem(filename: &str) -> Result<(Vec<Vec<i32>>, Vec<Vec<i32>>), io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let mut line_idx = 0;

    let n: usize = lines[line_idx].trim().parse().unwrap();
    line_idx += 2; // skip size line and empty line

    let mut distance = Vec::with_capacity(n);
    for _ in 0..n {
        let row: Vec<i32> = lines[line_idx].split_whitespace().map(|x| x.parse().unwrap()).collect();
        distance.push(row);
        line_idx += 1;
    }

    line_idx += 1; // skip empty line
    let mut flow = Vec::with_capacity(n);
    for _ in 0..n {
        let row: Vec<i32> = lines[line_idx].split_whitespace().map(|x| x.parse().unwrap()).collect();
        flow.push(row);
        line_idx += 1;
    }

    Ok((distance, flow))
}

fn calculate_cost(perm: &[usize], distance: &[Vec<i32>], flow: &[Vec<i32>]) -> i64 {
    let n = perm.len();
    let mut cost = 0i64;
    for i in 0..n {
        for j in 0..n {
            cost += flow[i][j] as i64 * distance[perm[i]][perm[j]] as i64;
        }
    }
    cost
}

fn main() -> io::Result<()> {
    let (distance, flow) = load_problem("silicon_spire.txt")?;

    // Generate all permutations for 4 facilities
    let perms = [
        [0, 1, 2, 3], [0, 1, 3, 2], [0, 2, 1, 3], [0, 2, 3, 1], [0, 3, 1, 2], [0, 3, 2, 1],
        [1, 0, 2, 3], [1, 0, 3, 2], [1, 2, 0, 3], [1, 2, 3, 0], [1, 3, 0, 2], [1, 3, 2, 0],
        [2, 0, 1, 3], [2, 0, 3, 1], [2, 1, 0, 3], [2, 1, 3, 0], [2, 3, 0, 1], [2, 3, 1, 0],
        [3, 0, 1, 2], [3, 0, 2, 1], [3, 1, 0, 2], [3, 1, 2, 0], [3, 2, 0, 1], [3, 2, 1, 0],
    ];

    let mut best_cost = i64::MAX;
    let mut best_perm = [0; 4];

    for perm in &perms {
        let cost = calculate_cost(perm, &distance, &flow);
        if cost < best_cost {
            best_cost = cost;
            best_perm = *perm;
        }
        println!("Permutation {:?}: Cost = {}", perm, cost);
    }

    println!("\nOptimal solution: {:?} with cost: {}", best_perm, best_cost);
    Ok(())
}
