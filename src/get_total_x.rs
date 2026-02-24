#[allow(dead_code)]
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
#[allow(dead_code)]
fn lcm(a: u32, b: u32) -> u32 {
    a / gcd(a, b) * b
}

#[allow(dead_code)]
fn get_total_x(a: &[u32], b: &[u32]) -> i32 {
    if a.is_empty() || b.is_empty() {
        return 0;
    }

    // LCM of all elements in a
    let lcm_a = a.iter().copied().reduce(lcm).unwrap();

    // GCD of all elements in b
    let gcd_b = b.iter().copied().reduce(gcd).unwrap();

    // Count multiples of lcm_a that divide gcd_b
    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}