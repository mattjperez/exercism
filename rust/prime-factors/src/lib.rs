pub fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();

    if let Some(x) = (2..=n).find(|x| n % x == 0) {
        result.push(x);
        result.append(&mut factors(n / x));
    }
    result
}
