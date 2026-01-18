fn biggerIsGreater(w: &str) -> String {
    let mut a: Vec<char> = w.chars().collect();
    let n = a.len();
    if n < 2 {
        return "no answer".to_string();
    }

    // 1) Find the rightmost pivot i where a[i] < a[i+1]
    let mut i = n - 2;
    while a[i] >= a[i + 1] {
        if i == 0 {
            return "no answer".to_string(); // fully non-increasing => no bigger permutation
        }
        i -= 1;
    }
    println!("i: {}", i);
    // 2) Find rightmost successor j > i such that a[j] > a[i]
    let mut j = n - 1;
    while a[j] <= a[i] {
        j -= 1;
    }
    println!("j: {}", j);

    // 3) Swap pivot with successor
    a.swap(i, j);

    // 4) Reverse the suffix (i+1..end). This is equivalent to sorting ascending here.
    a[i + 1..].reverse();

    a.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(biggerIsGreater("ab"), "ba".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(biggerIsGreater("bb"), "no.answer".to_string());
    }

    #[test]
    fn test_3() {
        assert_eq!(biggerIsGreater("hefg"), "hegf".to_string());
    }

    #[test]
    fn test_4() {
        assert_eq!(biggerIsGreater("dhck"), "dhkc".to_string());
    }

    #[test]
    fn test_5() {
        assert_eq!(biggerIsGreater("dkhc"), "hcdk".to_string());
    }

    #[test]
    fn test_6() {
        assert_eq!(biggerIsGreater("actqf"), "afcqt".to_string());
    }

}