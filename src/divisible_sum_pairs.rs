fn divisible_sum_pairs(_: i32, k: usize, ar: &[usize]) -> i32 {
    if ar.len() < 2 {
        return 0;
    }
    let mut acc = vec![0i32; k];
    for v in ar {
        let place = v % k;
        acc[place] += 1
    }
    let mut result = 0;
    for i in 1..((k + 1) / 2) {
        result += acc[k - i] * acc[i];
    }
    result +=  ((acc[0] - 1) * (acc[0])) / 2;
    if k % 2 == 0 {
        result += ((acc[k / 2] - 1) * (acc[k / 2])) / 2
    }
    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 6;
        let ar = vec![1, 2, 3, 4, 5, 6];
        let k = 5;
        assert_eq!(divisible_sum_pairs(n, k, &ar), 3);
    }

    #[test]
    fn test_2() {
        let n = 6;
        let ar = vec![1, 3, 2, 6, 1, 2];
        let k = 3;
        assert_eq!(divisible_sum_pairs(n, k, &ar), 5);
    }

    #[test]
    fn test_3() {
        let n = 4;
        let ar = vec![0, 2, 4, 8, 6, 10];
        let k = 4;
        assert_eq!(divisible_sum_pairs(n, k, &ar), 6);
    }
}
