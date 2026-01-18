fn build_string_cost(n: usize, a: i32, b: i32, s: &str) -> i32 {
    let mut dp = vec![0; n];
    dp[0] = a;

    let mut last_l = 0;

    for k in 1..n {
        dp[k] = dp[k - 1] + a;
        let mut l = last_l + 1;

        while l > 0 {
            let start = k + 1 - l;
            let cur = &s[start..=k];
            let prefix = &s[..start];

            if prefix.find(cur).is_some() {
                dp[k] = dp[k].min(dp[k - l] + b);
                break;
            } else {
                l -= 1;
            }
        }

        last_l = l;
    }

    dp[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(build_string_cost(9, 4, 5, "aabaacaba"), 26);
    }

    #[test]
    fn test_2() {
        assert_eq!(build_string_cost(9, 8, 9, "bacbacacb"), 42);
    }
}