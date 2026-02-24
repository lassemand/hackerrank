fn migratory_birds(arr: &[usize]) -> usize {
    let mut counts = vec![0usize; 5];
    for i in arr {
        counts[i-1] += 1;
    }

    let max = counts.iter().max().unwrap();
    counts.iter().position(|f| f == max).unwrap() + 1
}

#[cfg(test)]
mod tests {
    use crate::migratory_birds::migratory_birds;

    #[test]
    fn test_1() {
        let arr = vec![1, 2];
        assert_eq!(migratory_birds(&arr), 1);
    }
}
