const MOD: i64 = 1_000_000_007;


fn pair_palindrome(i: usize, j: usize, count: &Vec<[i32; 26]>) -> i64 {
    if i >= j {
        return 0;
    }

    let mut sum = 0i64;
    for k in 0..26 {
        let diff = (count[j][k] - count[i - 1][k]) as i64;
        sum = (sum + diff * (diff - 1) / 2) % MOD;
    }
    sum
}

fn short_palindrome(s: &str) -> i64 {
    let n = s.len();
    let chars: Vec<usize> = s.bytes().map(|c| (c - b'a') as usize).collect();

    // Prefix counts: count[i] = counts in s[0..i)
    let mut count = vec![[0i32; 26]; n + 1];
    for i in 1..=n {
        count[i] = count[i - 1];
        count[i][chars[i - 1]] += 1;
    }

    // most_recent[i] = last-seen positions BEFORE i (i.e., from prefix s[0..i-1])
    let mut most_recent = vec![[0i32; 26]; n + 1];
    let mut last = [0i32; 26];
    for i in 1..=n {
        most_recent[i] = last;                 // snapshot before update
        last[chars[i - 1]] = i as i32;         // update for future indices
    }

    let mut result = vec![0i64; n + 1];
    let mut cache = vec![[0i64; 26]; n + 1];
    let mut total = 0i64;

    for i in 1..=n {
        let c = chars[i - 1];
        let index = most_recent[i][c] as usize;

        if index == 0 {
            continue;
        }

        result[i] = result[index]
            + (count[index][c] as i64 - 1) * pair_palindrome(index, i - 1, &count)
            + pair_palindrome(index + 1, i - 1, &count);

        result[i] %= MOD;

        for k in 0..26 {
            let left = (count[i - 1][k] - count[index - 1][k]) as i64;

            result[i] = (result[i] + cache[index][k] * left) % MOD;

            cache[i][k] = (cache[index][k]
                + (count[index][c] as i64 - 1) * left
                + (count[i - 1][k] - count[index][k]) as i64)
                % MOD
        }

        total = (total + result[i]) % MOD;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let s = "kkkkkkz";
        assert_eq!(short_palindrome(s), 15);
    }

    #[test]
    fn test_2() {
        let s = "ghhggh";
        assert_eq!(short_palindrome(s), 4);
    }
    #[test]
    fn test_3() {
        let s = "dfdf";
        assert_eq!(short_palindrome(s), 0);
    }
}