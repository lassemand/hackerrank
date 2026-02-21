fn miniMaxSum(arr: &[i32]) {
    let (sum, min, max) = arr.iter().fold(
    (0, i32::MAX, i32::MIN),
    |(sum, min, max), &x| {
        (
            sum + x,
            min.min(x),
            max.max(x),
        )
    });
    print!("{} {}", sum - max, sum - min)

}
