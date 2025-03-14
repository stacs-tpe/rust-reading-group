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

    // Obnoxious banner
    println!("Welcome to MY_PRIMES, v1.1");
    println!("A blazingly fast Sieve of Eratosthenes, written in Rust");

    // Find and print the primes
    println!("Primes up to {n}:");
    for p in primes_up_to(n) {
        print!("{p} ");
    }
    println!();  // Clear line before returning prompt
}

fn primes_up_to(max: usize) -> Vec<usize> {
    // whether i is prime or not
    // TODO: shrink this with a bitmask, since 'bool' uses 8 bits
    let mut is_prime = vec![true; max];

    // List of primes to be returned
    let mut primes: Vec<usize> = Vec::new();

    // Go through all possible primes
    for p in 2..max {
        // Skip known primes
        if !is_prime[p] {
            continue;
        }

        // If p was not marked as non-prime in a previous iteration, then it is
        // not a multiple of any lower prime, so must itself be prime.
        primes.push(p);

        // Mark all its multiples as non-prime
        for k in 2..=(max-1)/p {
            is_prime[p * k] = false;
        }
    }

    primes
}
