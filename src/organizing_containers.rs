fn organizing_containers(container: &[Vec<i32>]) -> String {
    let mut row_sums: Vec<i32> = container
        .iter()
        .map(|row| row.iter().sum())
        .collect();

    let n = container.len();
    let mut col_sums: Vec<i32> = (0..n)
        .map(|j| container.iter().map(|row| row[j]).sum())
        .collect();

    row_sums.sort_unstable();
    col_sums.sort_unstable();

    if row_sums == col_sums {
        "Possible".to_string()
    } else {
        "Impossible".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_container_possible() {
        let container = vec![vec![5]];
        assert_eq!(organizing_containers(&container), "Possible");
    }

    #[test]
    fn test_single_container_zero() {
        let container = vec![vec![0]];
        assert_eq!(organizing_containers(&container), "Possible");
    }

    #[test]
    fn test_simple_possible() {
        // Container capacities = [3,3]
        // Ball type totals = [3,3]
        let container = vec![
            vec![1, 2],
            vec![2, 1],
        ];
        assert_eq!(organizing_containers(&container), "Possible");
    }

    #[test]
    fn test_simple_impossible() {
        // Container capacities = [3,3]
        // Ball type totals = [1,5]
        let container = vec![
            vec![0, 3],
            vec![1, 2],
        ];
        assert_eq!(organizing_containers(&container), "Impossible");
    }

    #[test]
    fn test_hackerrank_sample_possible() {
        let container = vec![
            vec![1, 1],
            vec![1, 1],
        ];
        assert_eq!(organizing_containers(&container), "Possible");
    }

    #[test]
    fn test_hackerrank_sample_impossible() {
        let container = vec![
            vec![0, 2],
            vec![1, 1],
        ];
        assert_eq!(organizing_containers(&container), "Impossible");
    }

    #[test]
    fn test_three_containers_possible() {
        let container = vec![
            vec![1, 3, 1],
            vec![2, 1, 2],
            vec![3, 3, 3],
        ];
        assert_eq!(organizing_containers(&container), "Impossible");
    }

    #[test]
    fn test_three_containers_impossible() {
        let container = vec![
            vec![1, 2, 3],
            vec![3, 3, 3],
            vec![2, 2, 2],
        ];
        assert_eq!(organizing_containers(&container), "Impossible");
    }

    #[test]
    fn test_all_zeros() {
        let container = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(organizing_containers(&container), "Possible");
    }

    #[test]
    fn test_large_balanced() {
        let container = vec![
            vec![10, 0, 0],
            vec![0, 10, 0],
            vec![0, 0, 10],
        ];
        assert_eq!(organizing_containers(&container), "Possible");
    }

    #[test]
    fn test_large_unbalanced() {
        let container = vec![
            vec![5, 5, 0],
            vec![0, 5, 5],
            vec![5, 0, 5],
        ];
        assert_eq!(organizing_containers(&container), "Possible");
    }
}
