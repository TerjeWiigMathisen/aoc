use devtimer::DevTime;

fn is_prime(value: usize) -> bool {
    (2..value).all(|i| value % i != 0)
}

fn main() {
    let mut devtime = DevTime::new_simple();
    devtime.start();

    let primes = (2..250_001).filter(|i| is_prime(*i)).collect::<Vec<_>>();
    devtime.stop();
    println!("{count}", count = primes.len());
    println!("Total time {} us",devtime.time_in_micros().unwrap());

}