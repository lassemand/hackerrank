fn cost(B: &[i32]) -> i32 {
    let n = B.len();
    if n <= 1 {
        return 0;
    }

    // dp[0] = max cost if A[i] = 1
    // dp[1] = max cost if A[i] = B[i]
    let mut dp = [0i32, 0i32];

    for i in 1..n {
        let prev0 = dp[0];
        let prev1 = dp[1];

        // A[i] = 1
        dp[0] = std::cmp::max(
            prev0,
            prev1 + (1 - B[i - 1]).abs(),
        );

        // A[i] = B[i]
        dp[1] = std::cmp::max(
            prev0 + (B[i] - 1).abs(),
            prev1 + (B[i] - B[i - 1]).abs(),
        );
    }

    std::cmp::max(dp[0], dp[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_size() {
        assert_eq!(cost(&[10]), 0);
    }

    #[test]
    fn test_two_equal() {
        assert_eq!(cost(&[5, 5]), 4);
    }

    #[test]
    fn test_two_decreasing() {
        assert_eq!(cost(&[10, 1]), 9);
    }

    #[test]
    fn test_all_ones() {
        assert_eq!(cost(&[1, 1, 1, 1, 1]), 0);
    }

    #[test]
    fn test_strictly_increasing() {
        assert_eq!(cost(&[1, 2, 3, 4, 5]), 8);
    }

    #[test]
    fn test_strictly_decreasing() {
        assert_eq!(cost(&[5, 4, 3, 2, 1]), 8);
    }

    #[test]
    fn test_alternating_high_low() {
        assert_eq!(
            cost(&[1, 100, 1, 100, 1, 100]),
            495
        );
    }

    #[test]
    fn test_large_flat() {
        assert_eq!(
            cost(&[100000, 100000, 100000, 100000]),
            299997
        );
    }

    #[test]
    fn test_peak_middle() {
        assert_eq!(
            cost(&[1, 2, 100, 2, 1]),
            198
        );
    }

    #[test]
    fn test_valley_middle() {
        assert_eq!(
            cost(&[100, 2, 1, 2, 100]),
            198
        );
    }

    #[test]
    fn test_random_small() {
        assert_eq!(
            cost(&[5, 1, 4, 2, 3]),
            12
        );
    }

    #[test]
    fn test_zigzag_medium() {
        assert_eq!(
            cost(&[10, 1, 10, 1, 10, 1, 10]),
            54
        );
    }

    #[test]
    fn test_zigzag() {
        assert_eq!(
            cost(&[10, 1, 1, 10]),
            18
        );
    }
}
