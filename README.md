# SiliconSpire QAP Solver (Rust)

A Rust command-line heuristic for the Quadratic Assignment Problem. It combines Grey Wolf Optimization (GWO), a continuous-position-to-permutation decoder, and swap-based Tabu Search.

The bundled cleanroom instance is illustrative. This repository does not demonstrate a deployed semiconductor-layout system, industrial savings, or benchmark performance.

## Build and run

```sh
cargo build --release
cargo run --release -- --input-file silicon_spire.txt
cargo run --release -- --input-file silicon_spire.txt --seed 42
```

An instance file starts with a positive integer `n`, then contains an `n × n` distance matrix and an `n × n` flow matrix. Each matrix row must contain exactly `n` integers.

The parser rejects missing matrix values, malformed rows, and non-whitespace data after the flow matrix. Assignments are checked as permutations of `0..n-1`; objective costs that overflow signed 64-bit arithmetic return an error.

The objective for an assignment `p` is:

```text
sum(i = 0..n-1) sum(j = 0..n-1) flow[i][j] * distance[p[i]][p[j]]
```

## Options

| Option | Default | Meaning |
| --- | ---: | --- |
| `--input-file FILE` | `silicon_spire.txt` | QAP instance path |
| `--pack-size N` | `30` | GWO population; at least three candidates are required |
| `--max-iterations N` | `100` | GWO iterations; must be positive |
| `--ts-iterations N` | `50` | Tabu iterations per invocation |
| `--tabu-tenure N` | `10` | Tabu-list length; must be positive |
| `--seed N` | generated | Optional unsigned 64-bit seed; the selected seed is printed for repeat runs |

For a brief usage summary, run `cargo run -- --help`.

## Checks

```sh
cargo fmt --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
```

## Scope and limitations

- GWO and Tabu Search are randomized heuristics; they do not guarantee an optimal assignment.
- The best assignment is retained across generations. After Tabu Search changes it, the solver re-encodes its continuous position before the next GWO iteration.
- Pass `--seed N` to repeat a run with the same build and input. Without it, the solver generates and prints a seed that can be reused. Reproducibility across different dependency versions or platforms is not guaranteed.
- The included small instance is a demonstration, not evidence of performance on larger QAP benchmark suites.
- No exact-solver comparison, industrial data, or real-world savings analysis is included.

## Attribution

This is an educational optimization prototype. The cleanroom narrative and included data are illustrative. See the source and Git history for implementation details.
