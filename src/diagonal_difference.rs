fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let pos = (0..n).fold(0, |acc, i| acc + arr[i][i]);
    let neg = (0..n).fold(0, |acc, i| acc + arr[n - i][i]);
    (pos - neg).abs()
}