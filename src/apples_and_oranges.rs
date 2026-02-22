fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut apple_count = 0;
    let mut orange_count = 0;

    for i in apples {
        if i + a >= s && i+a <= t {
            apple_count += 1;
        }
    }
    for i in oranges {
        if b + i >= s && b + i <= t {
            orange_count += 1;
        }
    }

    println!("{}", apple_count);
    println!("{}", orange_count);
}