pub fn abbreviation(a: &str, b: &str) -> String {
    let a = a.as_bytes();
    let b = b.as_bytes();

    let m = b.len();
    let mut dp = vec![false; m + 1];
    dp[0] = true;

    let mut max_j = 0;

    for &ch in a {
        let mut new_max = max_j;

        for j in (0..=max_j).rev() {
            if !dp[j] { continue; }

            if j < m && ch.to_ascii_uppercase() == b[j] {
                dp[j + 1] = true;
                new_max = new_max.max(j + 1);
            }

            if !ch.is_ascii_lowercase() {
                dp[j] = false;
            }
        }

        max_j = new_max;
    }

    if dp[m] { "YES".into() } else { "NO".into() }
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_1() {
    assert_eq!(abbreviation("daBcd", "ABC"), "YES")
}

#[test]
fn test_2() {
    assert_eq!(abbreviation("AbcDE", "ABDE"), "YES")
}

#[test]
fn test_3() {
    assert_eq!(abbreviation("AbcDE", "AFDE"), "NO")
}

#[test]
fn test_4() {
    assert_eq!(abbreviation("abc", "A"), "YES")
}

#[test]
fn test_5() {
    assert_eq!(abbreviation("abc", "BB"), "NO")
}

#[test]
fn test_6() {
    assert_eq!(abbreviation("abc", "D"), "NO")
}

#[test]
fn test_7() {
    assert_eq!(abbreviation("aA", "A"), "YES")
}

#[test]
fn test_8() {
    assert_eq!(abbreviation("aBcD", "AB"), "NO")
}
#[test]
fn test_9() {
    assert_eq!(abbreviation("ZGW", "ZG"), "NO")
}

#[test]
fn test_10() {
    assert_eq!(abbreviation("ZGAwZGB", "ZGB"), "NO")
}

#[test]
fn test_11() {
    assert_eq!(abbreviation("ab", "AB"), "YES")
}
}