# Card probability simulation

Choose two cards from a deck without replacement.

What is the probability that you draw *at least* one diamond or *exactly* one 7?

This program calculates that probability.

## Run
`cargo run --release [num_experiments: u32] (num_threads: u8)`

## Cool crates
- [rand](https://crates.io/crates/rand): random generation
- [rayon](https://crates.io/crates/rayon): parallelization
- [indicatif](https://crates.io/crates/indicatif): progress bar visualization
- [num](https://crates.io/crates/num): fraction calculation (probably unnecessary)
