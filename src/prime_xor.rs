const MOD: usize = 1_000_000_007;
const MAX_XOR: usize = 8192;
const MIN_VALUE: usize = 3500;
const MAX_VALUE: usize = 4500;


pub fn prime_xor(a: Vec<usize>) -> usize {
    let mut value_count = [0usize; MAX_VALUE - MIN_VALUE + 1];
    let mut xor_count = [0usize; MAX_XOR];
    let mut delta = [0usize; MAX_XOR];

    // Count frequencies
    for v in a {
        value_count[v - MIN_VALUE] += 1;
    }

    xor_count[0] = 1;

    // Process each value
    for (vi, &count) in value_count.iter().enumerate() {
        if count == 0 {
            continue;
        }

        let v = MIN_VALUE + vi;

        delta.fill(0);

        let odd = (count + 1) / 2;
        let even = count / 2;

        for i in 0..MAX_XOR {
            if xor_count[i] == 0 {
                continue;
            }

            if odd > 0 {
                let idx = i ^ v;
                delta[idx] = (delta[idx] + xor_count[i] * odd) % MOD;
            }

            if even > 0 {
                delta[i] = (delta[i] + xor_count[i] * even) % MOD;
            }
        }

        for i in 0..MAX_XOR {
            if delta[i] != 0 {
                xor_count[i] = (xor_count[i] + delta[i]) % MOD;
            }
        }
    }

    // Sieve primes up to MAX_XOR
    let mut is_composite = [false; MAX_XOR];
    for i in 2..MAX_XOR {
        let mut j = i * i;
        while j < MAX_XOR {
            is_composite[j] = true;
            j += i;
        }
    }

    // Sum prime XOR counts
    let mut result = 0usize;
    for i in 2..MAX_XOR {
        if !is_composite[i] && xor_count[i] != 0 {
            result = (result + xor_count[i]) % MOD;
        }
    }

    result
}
#[cfg(test)]
mod tests {
    use super::*;

    // ---------- Basic correctness ----------
    #[test]
    fn test_single_prime() {
        let arr = vec![2];
        assert_eq!(prime_xor(arr), 1); // XOR=2 (prime)
    }

    #[test]
    fn test_1() {
        let arr = vec![3511, 3671, 4153];
        assert_eq!(prime_xor(arr), 4);
    }
}