fn bon_appetit(bill: &[i32], k: usize, b: i64) {
    let sum: i64 = bill.iter().map(|&x| x as i64).sum();
    let expected_value = (sum - bill[k] as i64) / 2;
    if expected_value == b {
        print!("Bon Appetit")
    } else {
        print!("{}", expected_value - b)
    }

}