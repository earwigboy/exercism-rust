pub fn factors(n: u64) -> Vec<u64> {
    let mut n1 = n;
    let mut factors: Vec<u64> = Vec::new();

    for p in (2..).take_while(|i| i * i <= n) {
        while n1 % p == 0 {
            factors.push(p);
            n1 /= p;
        }
    }
    if n1 > 1 {
        factors.push(n1)
    }
    factors
}
