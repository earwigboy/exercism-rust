pub fn nth(n: u32) -> u32 {
    fn is_prime(n: u32) -> bool {
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let max = (n as f64).sqrt() as u32;
        for i in 3..=max {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    (2..).filter(|&x| is_prime(x)).nth(n as usize).unwrap_or(0)
}
