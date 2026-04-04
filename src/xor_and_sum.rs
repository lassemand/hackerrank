fn xor_and_sum(mut a: String, b: String) -> i64 {
    const N: usize = 414_159;
    const SHIFT: usize = 314_159;
    const MOD: i64 = 1_000_000_007;

    if a.len() < N {
        let zeros = "0".repeat(N - a.len());
        a = zeros + &a;
    }

    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    let mut two_power = vec![1i64; N + SHIFT + 2];
    for i in 1..two_power.len() {
        two_power[i] = (two_power[i - 1] * 2) % MOD;
    }

    let mut a_partial = vec![0i64; N + 1];

    for i in 0..N {
        let bit = (a_bytes[N - 1 - i] - b'0') as i64;
        a_partial[i + 1] =
            (a_partial[i] + bit * two_power[i]) % MOD;
    }

    let mut result: i64 = 0;

    for i in 0..=SHIFT {
        result += a_partial[i]
            + a_partial[N]
            - a_partial[i + b.len()];

        result %= MOD;
        if result < 0 {
            result += MOD;
        }
    }

    let mut cache: Vec<[i64; 2]> = vec![[0, 0]];

    for i in 0..=SHIFT {
        let bit = (a_bytes[N - 1 - i] - b'0') as i64;

        cache[0][0] =
            (cache[0][0] + ((bit ^ 0) * two_power[i]) % MOD) % MOD;

        cache[0][1] =
            (cache[0][1] + ((bit ^ 1) * two_power[i]) % MOD) % MOD;
    }

    // ---- main loop over b ----
    for i in 0..b.len() {
        let bbit = (b_bytes[b.len() - 1 - i] - b'0') as usize;

        result = (result + cache[i][bbit]) % MOD;
        if result < 0 {
            result += MOD;
        }

        let abit = (a_bytes[N - 1 - i] - b'0') as i64;

        let mut zero =
            cache[i][0] - ((abit ^ 0) * two_power[i]) % MOD;

        let add_pow = two_power[i + SHIFT + 1];

        let mut one =
            cache[i][1]
                - ((abit ^ 1) * two_power[i]) % MOD
                + add_pow;

        zero %= MOD;
        one %= MOD;

        if zero < 0 {
            zero += MOD;
        }
        if one < 0 {
            one += MOD;
        }

        cache.push([zero, one]);
    }

    (result % MOD + MOD) % MOD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let a = "10";
        let b = "1010";
        assert_eq!(xor_and_sum(a.to_string(), b.to_string()), 489429555);
    }
}