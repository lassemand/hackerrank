const MOD: i64 = 1_000_000_007;

pub fn countArray(n: i32, k: i32, x: i32) -> i64 {
    // Return the number of ways to fill in the array.
    let mut valid: i64 = if x == 1 { 0 } else { 1 };
    let mut all: i64 = 1;
    for _ in 2..n {
        all = all * (k - 1) as i64 % MOD;
        valid = (all - valid + MOD) % MOD;
    }
    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_examples() {
        // Examples that can be enumerated manually
        assert_eq!(countArray(4, 3, 2), 3);
    }

    #[test]
    fn test_k_equals_2() {
        // Only values {1,2}, no two adjacent equal
        assert_eq!(countArray(3, 2, 1), 1);
        assert_eq!(countArray(3, 2, 2), 1);
        assert_eq!(countArray(4, 2, 1), 1);
        assert_eq!(countArray(4, 2, 2), 1);
    }

    #[test]
    fn test_large_k_small_n() {
        assert_eq!(countArray(3, 100000, 1), 99998);
        assert_eq!(countArray(3, 100000, 2), 99999);
    }

    #[test]
    fn test_x_not_one_behavior() {
        assert_eq!(countArray(4, 3, 2), 8);
        assert_eq!(countArray(4, 3, 3), 8);
    }

    #[test]
    fn test_large_n_small_k() {
        assert_eq!(countArray(10, 2, 1), 1);
        assert_eq!(countArray(10, 2, 2), 1);
    }

    #[test]
    fn test_mod_behavior() {
        // Large n to ensure modulo behavior
        let result = countArray(1_000_000, 3, 2);
        assert!(result >= 0);
        assert!(result < MOD);
    }

    #[test]
    fn test_symmetry_for_x() {
        // For x != 1, result should be identical
        let a = countArray(5, 5, 2);
        let b = countArray(5, 5, 4);
        let c = countArray(5, 5, 5);
        assert_eq!(a, b);
        assert_eq!(b, c);
    }
}