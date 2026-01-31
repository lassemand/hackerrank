use std::collections::LinkedList;


const VISIT: [(i32, i32); 8] = [
    (1, 0), (-1, 0),
    (0, 1), (0, -1),
    (1, 1), (1, -1),
    (-1, 1), (-1, -1),
];

fn connectedCell(matrix: &[Vec<i32>]) -> i32 {
    if matrix.len() == 0 {
        return 0;
    }
    let n = matrix.len();
    let m = matrix[0].len();
    let mut visited = vec![vec![false; m]; n];
    let mut max_region = 0;

    let mut queue = LinkedList::new();
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 1 && !visited[i][j] {
                visited[i][j] = true;
                let mut size = 1;
                queue.push_back((i, j));
                while let Some((x, y)) = queue.pop_front() {
                    for (dx, dy) in VISIT {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx >= 0 && ny >= 0 &&
                           (nx as usize) < n &&
                           (ny as usize) < m {

                            let nx = nx as usize;
                            let ny = ny as usize;

                            if matrix[nx][ny] == 1 && !visited[nx][ny] {
                                size += 1;
                                visited[nx][ny] = true;
                                queue.push_back((nx, ny));
                            }
                        }
                    }
                }
                if size > max_region {
                    max_region = size;
                }
            }
        }
    }
    max_region
}

#[cfg(test)]
mod tests {
    use crate::connected_cell_in_a_grid::connectedCell;
    use crate::construct::countArray;

    #[test]
    fn test_1() {
            let matrix = vec![
            vec![1, 1, 0, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
            vec![1, 0, 0, 0],
        ];

        // The largest connected region has size 5
        // (top-left cluster connected diagonally)
        let result = connectedCell(&matrix);

        assert_eq!(result, 5);
        assert_eq!(countArray(4, 3, 2), 3);
    }
}