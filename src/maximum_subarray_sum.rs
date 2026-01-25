use std::collections::BTreeSet;

pub fn maximum_sum(a: &[i64], m: i64) -> i64 {
    let mut max_mod_sum: i64 = 0;
    let mut cur_prefix_sum: i64 = 0;

    let mut prefix_set = BTreeSet::new();
    prefix_set.insert(0);

    for &value in a {
        cur_prefix_sum = (cur_prefix_sum + value) % m;

        max_mod_sum = max_mod_sum.max(cur_prefix_sum);

        if let Some(&higher) = prefix_set.range((cur_prefix_sum + 1)..).next() {
            let candidate = (cur_prefix_sum - higher).rem_euclid(m);
            max_mod_sum = max_mod_sum.max(candidate);
        }

        prefix_set.insert(cur_prefix_sum);
    }

    max_mod_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let a = vec![3, 3, 9, 9, 5];
        let m = 7;
        assert_eq!(maximum_sum(&a, m), 6);
    }

    #[test]
    fn test_2() {
        let a = vec![10];
        let m = 7;
        assert_eq!(maximum_sum(&a, m), 3);
    }

    #[test]
    fn test_3() {
        let a = vec![1, 2, 3];
        let m = 10;
        assert_eq!(maximum_sum(&a, m), 6);
    }

    #[test]
    fn test_4() {
        let a = vec![5, 6, 7];
        let m = 7;
        assert_eq!(maximum_sum(&a, m), 6);
    }

    #[test]
    fn test_5() {
        let a = vec![0, 0, 0];
        let m = 5;
        assert_eq!(maximum_sum(&a, m), 0);
    }

    #[test]
    fn test_6() {
        let a = vec![10, 9, 8, 7];
        let m = 11;
        assert_eq!(maximum_sum(&a, m), 10);
    }


}

