fn kaprekar_numbers(p: i32, q: i32) {
    let mut sq = p.pow(2) as i64;
    let init_2 = (p + 1).pow(2) as i64;
    let mut increase = init_2 - sq - 2;
    let mut candidates: Vec<i32> = Vec::new();
    for n in p..=q {
        let mut k = 1;
        let mut temp = n;
        while temp > 0 {
            k *= 10;
            temp /= 10;
        }

        let right = sq % k;
        let left = sq / k;

        if right > 0 && left + right == n as i64 {
            candidates.push(n);
        }
        increase += 2;
        sq += increase;
    }
    if candidates.is_empty() {
        println!("INVALID RANGE");
    } else {
        let output = candidates
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        kaprekar_numbers(1, 100)
    }
    #[test]
    fn test_1() {
        kaprekar_numbers(1, 99999)
    }

}

