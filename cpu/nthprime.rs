fn nth_prime(n: usize) -> i64 {
    let mut primes: Vec<i64> = vec![2];
    let mut x: i64 = 3;
    while primes.len() < n {
        let mut is_prime = true;
        for &p in &primes {
            if p * p > x { break; }
            if x % p == 0 { is_prime = false; break; }
        }
        if is_prime { primes.push(x); }
        x += 2;
    }
    primes[n - 1]
}

fn main() {
    println!("{}", nth_prime(1000000));
}
