fn plus_minus(arr: &[i32]) {
    if arr.len() == 0 {
        return
    }
    let (positive, negative, minus): (i64, i64, i64) = arr.iter().fold((0, 0, 0), |(p, n, z), i| {
        if *i < 0 {
            (p, n + 1, z)
        } else if *i > 0 {
            (p + 1, n, z)
        } else {
            (p, n, z + 1)
        }
    });

    let values: Vec<f64> = vec![positive as f64, negative as f64, minus as f64];
    let sum: f64 = values.iter().sum();

    for value in values {
        let division = value / sum as f64;
        println!("{:.6}", division)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr = vec![1, -1];
        plus_minus(&arr)
    }
}