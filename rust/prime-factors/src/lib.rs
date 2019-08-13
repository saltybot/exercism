pub fn factors(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::new();
    let mut num = n;

    while num % 2 == 0 {
        primes.push(2);
        num /= 2;
    }

    for i in 3..=((n as f64).sqrt() as u64) {
        if num % i == 0 {
            primes.push(i);
            num /= i;
        }
    }

    if num > 2 {
        primes.push(num)
    }

    primes
}
