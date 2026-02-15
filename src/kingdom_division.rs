use std::collections::{HashMap};


const MOD: i64 = 1_000_000_007;

pub fn kingdom_division(n: i32, roads: &[Vec<i32>]) -> i32 {
    let n = n as usize;
    if n == 0 { return 0; }
    if n == 1 { return 0; }

    let mut g = vec![Vec::<usize>::new(); n];
    for r in roads {
        let u = (r[0] - 1) as usize;
        let v = (r[1] - 1) as usize;
        g[u].push(v);
        g[v].push(u);
    }

    // build parent + traversal order (tree rooted at 0)
    let mut parent = vec![usize::MAX; n];
    let mut order = Vec::with_capacity(n);
    let mut stack = vec![0usize];
    parent[0] = 0;

    while let Some(u) = stack.pop() {
        order.push(u);
        for &v in &g[u] {
            if parent[v] == usize::MAX { // not visited yet
                parent[v] = u;
                stack.push(v);
            }
        }
    }

    // DP:
    // same[u] = ways when u has same color as parent
    // diff[u] = ways when u has different color from parent
    let mut same = vec![1i64; n];
    let mut diff = vec![1i64; n];

    for &u in order.iter().rev() {
        let mut prod_all = 1i64;
        let mut prod_diff = 1i64;

        for &v in &g[u] {
            if v == parent[u] { continue; } // Parent has already been visited
            let s = same[v];
            let d = diff[v];
            let t = (s + d) % MOD;

            prod_all = (prod_all * t) % MOD;
            prod_diff = (prod_diff * d) % MOD;
        }

        same[u] = prod_all;
        diff[u] = (prod_all - prod_diff + MOD) % MOD;
    }

    ((2 * diff[0]) % MOD) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_node() {
        let n = 1;
        let roads: Vec<Vec<i32>> = vec![];
        assert_eq!(kingdom_division(n, &roads), 0);
    }

    #[test]
    fn two_nodes() {
        let n = 2;
        let roads = vec![vec![1, 2]];
        // Both must have same color → 2 ways
        assert_eq!(kingdom_division(n, &roads), 2);
    }

    #[test]
    fn three_line() {
        // 1 - 2 - 3
        let n = 3;
        let roads = vec![vec![1, 2], vec![2, 3]];
        assert_eq!(kingdom_division(n, &roads), 2);
    }

    #[test]
    fn small_tree() {
        let n = 4;
        let roads = vec![
            vec![1, 2],
            vec![1, 3],
            vec![2, 4],
        ];
        assert_eq!(kingdom_division(n, &roads), 4);
    }
}}

