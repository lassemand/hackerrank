fn fair_cut(mut k: usize, arr: &[i32]) -> i64 {
    let n = arr.len();
    if n < 2 {
        return 0;
    }

    k = k.min(n - k);

    let mut a: Vec<i64> = arr.iter().map(|&x| x as i64).collect();
    a.sort_unstable();

    let mut result: i64 = 0;
    let mut j: i64 = n as i64; // number of remaining elements
    let mut kk: i64 = k as i64;

    for i in 0..(n / 2) {
        let low = a[i];
        let high = a[n - 1 - i];
        let diff = high - low;

        // Compare kk with j - kk (sizes of remaining groups)
        if kk > j - kk {
            result += diff * (j - kk);
            j -= 2;
            kk -= 2;
        } else {
            result += diff * kk;
            j -= 2;
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(fair_cut(1, &vec![1, 1, 2]), 1)
    }
    #[test]
    fn test_2() {
        assert_eq!(fair_cut(2, &vec![1, 1, 2, 3]), 4)
    }
    #[test]
    fn test_3() {
        assert_eq!(fair_cut(3, &vec![1, 1, 2, 3]), 3)
    }
    #[test]
    fn test_4() {
        assert_eq!(fair_cut(3, &vec![4, 5, 1, 2, 3]), 12)
    }

    #[test]
    fn test_5() {
        assert_eq!(fair_cut(11, &vec![691259308, 801371251, 345390019, 162749471, 998969126, 308205008, 430442891, 404642721, 532566673, 266540863, 702197285, 749105392, 775025448, 20453591, 582291534, 132855413, 747557193, 129094259, 474372133, 788391070]), 30481712493)
    }

    #[test]
    fn test_6() {
        assert_eq!(fair_cut(4, &vec![266681649, 727937104, 567502644, 955573039, 72596944, 347777854, 459994855, 477510595, 379479220, 994354886, 136619620, 122314175, 999247852, 536621456, 664892182, 683112373, 910813659, 339321017, 351973368, 784871915]), 18460346306
)
    }
}