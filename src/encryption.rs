use std::cmp::min;

fn encryption(s: &str) -> String {

    let filtered: String = s.chars().filter(|c| !c.is_whitespace()).collect();

    let len = filtered.len();
    let sqrt = (len as f64).sqrt();

    let mut rows = sqrt.floor() as usize;
    let columns = sqrt.ceil() as usize;

    if rows * columns < len {
        rows += 1;
    }
    
   (0..columns)
        .map(|col| {
            (0..rows)
                .filter_map(|row| {
                    let start = row * columns;
                    let end = min(start + columns, len as usize);
                    filtered[start..end]
                        .chars().nth(col)
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(encryption("haveaniceday"), "hae and via ecy".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(encryption("feedthedog"), "fto ehg ee dd".to_string());
    }

    #[test]
    fn test_3() {
        assert_eq!(encryption("chillout"), "clu hlt io".to_string());
    }
}

