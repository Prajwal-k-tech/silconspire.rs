# SiliconSpire.rs 🔬⚡

A high-performance Rust implementation of a hybrid metaheuristic algorithm for solving the Quadratic Assignment Problem (QAP) using Grey Wolf Optimizer (GWO) enhanced with Tabu Search.

## 🎯 Problem Statement

### The Silicon Spire Challenge

**Silicon Spire Dynamics** is building a cutting-edge semiconductor fabrication plant with four critical processing modules that need optimal placement in designated cleanroom bays. The goal is to minimize wafer pod travel distance to reduce processing time, energy consumption, and defect risk.

**The Four Modules:**
1. **Photolithography Bay** - Projects circuit patterns onto wafers
2. **Etching & Cleaning Station** - Removes material and cleans wafers  
3. **Deposition Chamber** - Adds ultra-thin material layers
4. **Metrology & Inspection Hub** - Inspects wafers for defects

**The Challenge:** Place these modules in four bays (Alpha, Beta, Gamma, Delta) to minimize total transport distance, potentially saving millions in production costs.

### The Quadratic Assignment Problem (QAP)

The QAP is a well-known NP-hard optimization problem that seeks to minimize:

```
Cost = Σᵢ Σⱼ flow[i][j] × distance[π(i)][π(j)]
```

Where `π` represents a permutation (assignment) of facilities to locations.

## 🐺 Algorithm Overview

This implementation uses a **hybrid metaheuristic** combining:

- **Grey Wolf Optimizer (GWO)**: Population-based global search inspired by wolf pack hunting behavior
- **Tabu Search (TS)**: Intensive local search with memory to avoid cycling
- **Largest Value Priority (LVP)**: Converts continuous GWO positions to discrete permutations

### Key Features

✅ **Hybrid Approach**: GWO for exploration + Tabu Search for exploitation  
✅ **Robust Implementation**: Comprehensive error handling and input validation  
✅ **Flexible CLI**: Configurable parameters for experimentation  
✅ **High Performance**: Optimized Rust implementation with release builds  
✅ **Proven Results**: Consistently finds optimal solutions on test instances  

## 🚀 Installation

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))

### Build from Source
```bash
git clone https://github.com/YOUR_USERNAME/silconspire.rs.git
cd silconspire.rs
cargo build --release
```

## 📊 Usage

### Basic Usage
```bash
cargo run --release
```

### With Custom Parameters
```bash
cargo run --release -- \
  --input-file my_problem.txt \
  --pack-size 50 \
  --max-iterations 200 \
  --ts-iterations 100 \
  --tabu-tenure 15
```

### Command Line Options

| Option | Description | Default |
|--------|-------------|---------|
| `--input-file` | Path to QAP problem file | `silicon_spire.txt` |
| `--pack-size` | Number of wolves in the pack | `30` |
| `--max-iterations` | Number of GWO iterations | `100` |
| `--ts-iterations` | Tabu Search iterations per hybridization | `50` |
| `--tabu-tenure` | Size of the tabu list | `10` |

### Help
```bash
cargo run --release -- --help
```

## 📝 Input File Format

The tool expects QAP instances in the following format:

```
4

0 10 15 20
10 0 35 25
15 35 0 30
20 25 30 0

0 90 120 80
90 0 40 50
120 40 0 70
80 50 70 0
```

- Line 1: Problem size `n`
- Empty line
- Next `n` lines: Distance matrix (n×n)
- Empty line  
- Next `n` lines: Flow matrix (n×n)

## 🔬 Algorithm Details

### Grey Wolf Optimizer
- **Population**: Pack of wolves with continuous position vectors
- **Hierarchy**: Alpha (best), Beta (second), Delta (third) guide the pack
- **Position Update**: Wolves adjust positions based on leader influences
- **Convergence Parameter**: `a` decreases linearly from 2 to 0

### Tabu Search Enhancement
- **Neighborhood**: 2-opt swaps (exchange any two positions)
- **Tabu List**: FIFO queue preventing recently visited moves
- **Aspiration Criterion**: Override tabu status for global best improvements
- **Intensification**: Applied to Alpha wolf after each GWO iteration

### Largest Value Priority (LVP)
Converts continuous position vectors to discrete permutations by sorting indices based on position values.

## 📈 Performance Results

### Silicon Spire 4×4 Instance
- **Optimal Solution Found**: `[0, 2, 1, 3]` 
- **Minimum Cost**: `17,600`
- **Consistency**: 100% success rate across multiple runs
- **Verification**: Confirmed optimal via exhaustive search

### Scalability
- Successfully handles larger instances (tested up to 6×6)
- Efficient memory usage with Rust's zero-cost abstractions
- Release builds provide significant performance improvements

## 🛠️ Development

### Testing
```bash
# Run basic tests
cargo test

# Performance test with release build
cargo build --release
./target/release/silconspire --pack-size 100 --max-iterations 500

# Verify solution quality
cargo run --release -- --pack-size 50 --max-iterations 100
```

### Dependencies
- `clap` - Command-line argument parsing
- `rand` - Random number generation

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit changes: `git commit -am 'Add feature'`
4. Push to branch: `git push origin feature-name`
5. Submit a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔬 Research Context

This implementation serves as a practical exploration of:
- **Metaheuristic Hybridization**: Combining global and local search strategies
- **Discrete Optimization**: Handling combinatorial problems with continuous algorithms
- **High-Performance Computing**: Leveraging Rust for scientific computing applications

## 📚 References

- Mirjalili, S., Mirjalili, S. M., & Lewis, A. (2014). Grey wolf optimizer. *Advances in Engineering Software*, 69, 46-61.
- Glover, F. (1986). Future paths for integer programming and links to artificial intelligence. *Computers & Operations Research*, 13(5), 533-549.
- Loiola, E. M., et al. (2007). A survey for the quadratic assignment problem. *European Journal of Operational Research*, 176(2), 657-690.

---

*Built with 🦀 Rust for maximum performance and reliability*
