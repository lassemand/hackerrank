fn suffix_array(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    if n == 0 {
        return vec![];
    }

    let mut sa: Vec<usize> = (0..n).collect();

    // rank[i] = rank of suffix starting at i
    // start with rank = byte value (0..255)
    let mut rank: Vec<i32> = s.iter().map(|&b| b as i32).collect();

    let mut tmp: Vec<i32> = vec![0; n];

    let mut k: usize = 1;
    while k < n {
        // Sort by (rank[i], rank[i+k]) using current ranks
        sa.sort_unstable_by(|&i, &j| {
            let a0 = rank[i];
            let b0 = rank[j];
            if a0 != b0 {
                return a0.cmp(&b0);
            }
            let a1 = if i + k < n { rank[i + k] } else { -1 };
            let b1 = if j + k < n { rank[j + k] } else { -1 };
            a1.cmp(&b1)
        });
        // Re-rank
        tmp[sa[0]] = 0;
        let mut r: i32 = 0;
        for idx in 1..n {
            let i = sa[idx - 1];
            let j = sa[idx];

            let prev = (rank[i], if i + k < n { rank[i + k] } else { -1 });
            let curr = (rank[j], if j + k < n { rank[j + k] } else { -1 });

            if prev != curr {
                r += 1;
            }
            tmp[j] = r;
        }

        // swap rank/tmp
        for i in 0..n {
            rank[i] = tmp[i];
        }

        if r as usize == n - 1 {
            break; // all ranks unique
        }
        k <<= 1;
    }

    sa
}

fn lcp_array(s: &[u8], sa: &[usize]) -> Vec<usize> {
    let n = s.len();
    if n == 0 {
        return vec![];
    }

    let mut rank = vec![0usize; n];
    for (i, &p) in sa.iter().enumerate() {
        rank[p] = i;
    }
    println!("rank {:?}", rank);

    // Kasai: lcp between sa[i] and sa[i+1], store at lcp[i] for i in 0..n-1
    let mut lcp = vec![0usize; n];
    let mut k = 0usize;

    for i in 0..n {
        let ri = rank[i];
        if ri == n - 1 {
            k = 0;
            continue;
        }
        let j = sa[ri + 1];
        while i + k < n && j + k < n && s[i + k] == s[j + k] {
            k += 1;
        }
        lcp[ri] = k;
        if k > 0 {
            k -= 1;
        }
    }

    lcp
}

fn max_value(t: &str) -> i64 {
    let s = t.as_bytes();
    let n = s.len();
    if n == 0 {
        return 0;
    }

    let sa = suffix_array(s);
    let lcp = lcp_array(s, &sa);

    // Match the Python logic:
    // ans starts at n (single-character substrings occur at least once, and length*n?? actually n is max length*freq baseline)
    let mut ans: i64 = n as i64;

    // left/right arrays over indices 0..n-1 (same as Python)
    let mut left = vec![-1i32; n];
    let mut right = vec![n as i32; n];

    // Previous smaller element (strictly smaller) on lcp using monotonic stack with >= pop
    let mut st: Vec<usize> = Vec::new();
    st.push(0);
    println!("Lcp: {:?}", lcp);
    for i in 1..n {

        while let Some(&top) = st.last() {
            println!("st: {:?}", st);
            if lcp[top] >= lcp[i] {
                st.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = st.last() {
            left[i] = top as i32;
        }
        println!("Done: {}", i );
        st.push(i);
    }

    // Next smaller element to the right
    st.clear();
    st.push(n - 1);
    for i in (0..=(n - 2)).rev() {
        while let Some(&top) = st.last() {
            if lcp[top] >= lcp[i] {
                st.pop();
            } else {
                break;
            }
        }
        if let Some(&top) = st.last() {
            right[i] = top as i32;
        }
        st.push(i);
    }

    for i in 0..n {
        let height = lcp[i] as i64;
        let width = (right[i] - left[i]) as i64;
        let val = height * width;
        if val > ans {
            ans = val;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let s = "";
        assert_eq!(max_value(s), 0);
    }

    #[test]
    fn test_1() {
        let s = "aaaaaa";
        assert_eq!(max_value(s), 12);
    }

    #[test]
    fn test_2() {
        let s = "abcabcddd";
        assert_eq!(max_value(s), 9);
    }

    #[test]
    fn test_3() {
        let a = "abaabbaaaaabbbbbbbabbbbaaaaaabbaababaaabaaabbbbaabbaaaaaaaaabbaaaabbababbaaabbababbaabaabbbbabbbaaaaaabbaaabbaaaaaaabbababaabbbbbbbbbbbbbbaabbabbbbaaababbaaaababbbbaaabbbbbaaabbabbabbbababbbbbaaaabababbbbbaaaaabababbbbabbbabaaababbaabbbbabbaabbaaaaabbbabaababbabbaababbbababaabaabbbaaababbabbbaabbaaababbbbaaaaabaababaabababbaabaabaabbbabbaabaaababbaabbbabbaaaabaaabbbbbbbabbabbaaaaabbaabbabbabaaabbbabababbbbbabaaabababbbbbababababababababbaababababababbbababaaabbaaababababbbaabbbabababaaabbaaaaaabaaababbbbbaaaaabaaaababaabaabbbabbbbbaaaaabaabbabbbabaababbbabbbababbaababaaabbbbaaaaaaabbabababaaaaaababbbbbabaabaababbabaaabbbbabaaaaaabbbbabbbbababaaaaaababbaababbabbaaaabaaaababbabbbbbbbabbbbaaabbabaaaaababbbaaaabbbaabbaaaaaaaaababaabbababbabbbaabbbabbbbabbaababbaaaababbbaaaababaaaabaabbabaaabaabaaabbbbbbbabbbabbbbbabbbbbbbbaabaaabbbabababaabbaaaabbabababbaabbaaabbbbaabaaabaabbaabbaabbabbbababababbaabbaabbbabbbababaaabbbabaaaabbbaaaaabbbaaababbbbbaabbaababbaaaabaabaabaabbbbbaaaabbbbaaabaabaaabaabbaaaaabbbbbbbabbbbaaaaaabbaababaaabaaabbbbaabbaaaaaaaaabbaaaabbababbaaabbababbaabaabbbbabbbaaaaaabbaaabbaaaaaaabbababaabbbbbbbbbbbbbbaabbabbbbaaababbaaaababbbbaaabbbbbaaabbabbabbbababbbbbaaaabababbbbbaaaaabababbbbabbbabaaababbaabbbbabbaabbaaaaabbbabaababbabbaababbbababaabaabbbaaababbabbbaabbaaababbbbaaaaabaababaabababbaabaabaabbbabbaabaaababbaabbbabbaaaabaaabbbbbbbabbabbaaaaabbaabbabbabaaabbbabababbbbbabaaabababbbbbababababababababbaababababababbbababaaabbaaababababbbaabbbabababaaabbaaaaaabaaababbbbbaaaaabaaaababaabaabbbabbbbbaaaaabaabbabbbabaababbbabbbababbaababaaabbbbaaaaaaabbabababaaaaaababbbbbabaabaababbabaaabbbbabaaaaaabbbbabbbbababaaaaaababbaababbabbaaaabaaaababbabbbbbbbabbbbaaabbabaaaaababbbaaaabbbaabbaaaaaaaaababaabbababbabbbaabbbabbbbabbaababbaaaababbbaaaababaaaabaabbabaaabaabaaabbbbbbbabbbabbbbbabbbbbbbbaabaaabbbabababaabbaaaabbabababbaabbaaabbbbaabaaabaabbaabbaabbabbbababababbaabbaabbbabbbababaaabbbabaaaabbbaaaaabbbaaababbbbbaabbaababbaaaabaabaabaabbbbbaaaabbbbaaabaabaaabaabbaaaaabbbbbbbabbbbaaaaaabbaababaaabaaabbbbaabbaaaaaaaaabbaaaabbababbaaabbababbaabaabbbbabbbaaaaaabbaaabbaaaaaaabbababaabbbbbbbbbbbbbbaabbabbbbaaababbaaaababbbbaaabbbbbaaabbabbabbbababbbbbaaaabababbbbbaaaaabababbbbabbbabaaababbaabbbbabbaabbaaaaabbbabaababbabbaababbbababaabaabbbaaababbabbbaabbaaababbbbaaaaabaababaabababbaabaabaabbbabbaabaaababbaabbbabbaaaabaaabbbbbbbabbabbaaaaabbaabbabbabaaabbbabababbbbbabaaabababbbbbababababababababbaababababababbbababaaabbaaababababbbaabbbabababaaabbaaaaaabaaababbbbbaaaaabaaaababaabaabbbabbbbbaaaaabaabbabbbabaababbbabbbababbaababaaabbbbaaaaaaabbabababaaaaaababbbbbabaabaababbabaaabbbbabaaaaaabbbbabbbbababaaaaaababbaababbabbaaaabaaaababbabbbbbbbabbbbaaabbabaaaaababbbaaaabbbaabbaaaaaaaaababaabbababbabbbaabbbabbbbabbaababbaaaababbbaaaababaaaabaabbabaaabaabaaabbbbbbbabbbabbbbbabbbbbbbbaabaaabbbabababaabbaaaabbabababbaabbaaabbbbaabaaabaabbaabbaabbabbbababababbaabbaabbbabbbababaaabbbabaaaabbbaaaaabbbaaababbbbbaabbaababbaaaabaabaabaabbbbbaaaabbbbaaabaabaaabaabbaaaaabbbbbbbabbbbaaaaaabbaababaaabaaabbbbaabbaaaaaaaaabbaaaabbababbaaabbababbaabaabbbbabbbaaaaaabbaaabbaaaaaaabbababaabbbbbbbbbbbbbbaabbabbbbaaababbaaaababbbbaaabbbbbaaabbabbabbbababbbbbaaaabababbbbbaaaaabababbbbabbbabaaababbaabbbbabbaabbaaaaabbbabaababbabbaababbbababaabaabbbaaababbabbbaabbaaababbbbaaaaabaababaabababbaabaabaabbbabbaabaaababbaabbbabbaaaabaaabbbbbbbabbabbaaaaabbaabbabbabaaabbbabababbbbbabaaabababbbbbababababababababbaababababababbbababaaabbaaababababbbaabbbabababaaabbaaaaaabaaababbbbbaaaaabaaaababaabaabbbabbbbbaaaaabaabbabbbabaababbbabbbababbaababaaabbbbaaaaaaabbabababaaaaaababbbbbabaabaababbabaaabbbbabaaaaaabbbbabbbbababaaaaaababbaababbabbaaaabaaaababbabbbbbbbabbbbaaabbabaaaaababbbaaaabbbaabbaaaaaaaaababaabbababbabbbaabbbabbbbabbaababbaaaababbbaaaababaaaabaabbabaaabaabaaabbbbbbbabbbabbbbbabbbbbbbbaabaaabbbabababaabbaaaabbabababbaabbaaabbbbaabaaabaabbaabbaabbabbbababababbaabbaabbbabbbababaaabbbabaaaabbbaaaaabbbaaababbbbbaabbaababbaaaabaabaabaabbbbbaaaabbbbaaabaabaaabaabbaaaaabbbbbbbabbbbaaaaaabbaababaaabaaabbbbaabbaaaaaaaaabbaaaabbababbaaabbababbaabaabbbbabbbaaaaaabbaaabbaaaaaaabbababaabbbbbbbbbbbbbbaabbabbbbaaababbaaaababbbbaaabbbbbaaabbabbabbbababbbbbaaaabababbbbbaaaaabababbbbabbbabaaababbaabbbbabbaabbaaaaabbbabaababbabbaababbbababaabaabbbaaababbabbbaabbaaababbbbaaaaabaababaabababbaabaabaabbbabbaabaaababbaabbbabbaaaabaaabbbbbbbabbabbaaaaabbaabbabbabaaabbbabababbbbbabaaabababbbbbababababababababbaababababababbbababaaabbaaababababbbaabbbabababaaabbaaaaaabaaababbbbbaaaaabaaaababaabaabbbabbbbbaaaaabaabbabbbabaababbbabbbababbaababaaabbbbaaaaaaabbabababaaaaaababbbbbabaabaababbabaaabbbbabaaaaaabbbbabbbbababaaaaaababbaababbabbaaaabaaaababbabbbbbbbabbbbaaabbabaaaaababbbaaaabbbaabbaaaaaaaaababaabbababbabbbaabbbabbbbabbaababbaaaababbbaaaababaaaabaabbabaaabaabaaabbbbbbbabbbabbbbbabbbbbbbbaabaaabbbabababaabbaaaabbabababbaabbaaabbbbaabaaabaabbaabbaabbabbbababababbaabbaabbbabbbababaaabbbabaaaabbbaaaaabbbaaababbbbbaabbaababbaaaabaabaabaabbbbbaaaabbbbaaabaabaa";
        assert_eq!(max_value(a), 9000);
    }
}