use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    for _ in 0..1000 {
        let sum_of_squares: u64 = (1..=1_000_000).map(|x| x * x).sum();
    }

    let elapsed_time = start_time.elapsed().as_secs_f64();
    println!("Total time for 1,000 runs: {} seconds", elapsed_time);
}
