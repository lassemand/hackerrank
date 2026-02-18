const MOD: i64 = 1_000_000_007;
fn substrings(n: &str) -> i64 {
    let bytes = n.as_bytes();

    let mut prev = 0i64;
    let mut total = 0i64;

    for (i, &b) in bytes.iter().enumerate() {
        let digit = (b - b'0') as i64;
        let curr = (prev * 10 + digit * (i as i64 + 1)) % MOD;
        total = (total + curr) % MOD;
        prev = curr;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let n = "42";
        assert_eq!(substrings(n), 48);
    }

    #[test]
    fn test_2() {
        let n = "972698438521";
        assert_eq!(substrings(n), 445677619);
    }

}