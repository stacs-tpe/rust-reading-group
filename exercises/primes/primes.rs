use std::env;

fn main() {
    // Get n from command line
    let args: Vec<String> = env::args().collect();
    let n = match args.get(1) {
        Some(arg) => match arg.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Argument n must be a positive integer");
                return;
            }
        },
        None => {
            println!("Usage: ./primes n");
            return;
        }
    };
    
    println!("Welcome to MY_PRIMES, v1.0");
    println!("A blazingly fast Sieve of Eratosthenes, written in Rust");

    println!("Primes up to {n}:");

    let primes = primes_up_to(n);
    for p in primes {
        print!("{p} ");
    }
    println!();
}

fn primes_up_to(max: u32) -> Vec<u32> {
    let max = max as usize;
    let mut is_prime = vec![true; max];  // whether i is prime
    let mut primes: Vec<u32> = Vec::new();

    // Go through all possible primes
    for p in 2..max {
        // If not prime, skip
        if !is_prime[p] {
            continue;
        }
        // This is prime!
        primes.push(p as u32);
        // Mark all its multiples as non-prime
        for k in 2..=(max-1)/p {
            is_prime[p * k] = false;
        }
    }
    
    primes
}
