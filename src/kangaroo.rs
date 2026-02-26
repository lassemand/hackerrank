fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    let c = x2 - x1;
    let d = v1 - v2;
    if d <= 0 || c % d != 0 {
        return "NO".to_string();
    }
    "YES".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO")
    }

    #[test]
    fn test_1() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES")
    }

}

