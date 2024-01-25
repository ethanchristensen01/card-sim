mod card;
use card::*;
use num::rational::*;
use num::ToPrimitive;
use rayon::prelude::*;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::ThreadPoolBuilder;
use std::env;

fn experiment_condition (cards: Vec<&Card>) -> bool {
    assert!(cards.len() == 2, "experiment expects two cards");
    cards.iter().any(|c| c.suit == Suit::Diamonds)
        || 
    cards.iter().filter(|c| c.rank == Rank::Seven).count() == 1
}

#[derive(Default)]
struct PosNegCount {
    positive: u64,
    negative: u64
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let num_experiments: u32 = args.get(0)
        .expect("Missing first argument: experiment count")
        .parse::<u32>()
        .expect("Couldn't parse experiment count");
    if let Some(num_threads) = args.get(1).map(|a| a.parse::<usize>()) {
        let num_threads = num_threads.expect("couldn't parse thread count");
        ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
    }
    let deck = card::Deck::default();
    
    let style = ProgressStyle::with_template(
        "{percent}% {wide_bar} [{elapsed_precise}]\n{bytes_per_sec}\n{eta} left"
    ).unwrap();
    
    let size = num_experiments;
    let range = 0..size;
    let count: PosNegCount = range.into_par_iter()
        .progress_with_style(style)
        .map_init(
            || rand::thread_rng(),
            |rng, _| deck.draw_random(rng, 2)
        )
        .map(
            experiment_condition 
        )
        .fold(
            || PosNegCount::default(),
            |acc, v| {
                if v {
                    PosNegCount { positive: acc.positive + 1, negative: acc.negative }
                } else {
                    PosNegCount { positive: acc.positive, negative: acc.negative + 1 }
                }
            }
        ).reduce(
            || PosNegCount::default(),
            |a, b| PosNegCount {
                positive: a.positive + b.positive,
                negative: a.negative + b.negative
            }
        );
    
    let total = count.positive + count.negative;
    let positive_rate = Ratio::new(count.positive, total).to_f64().unwrap();
    let negative_rate = Ratio::new(count.negative, total).to_f64().unwrap();
    
    println!("positive_rate {0:.3}%\n negative rate {1:.3}%", positive_rate * 100.0, negative_rate * 100.0);
    
}
