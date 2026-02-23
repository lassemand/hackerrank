fn staircase(n: usize) {
    for i in 1..=n {
        let value : String = std::iter::repeat(' ').take(n - i)
            .chain(std::iter::repeat('#').take(i))
            .collect();
        println!("{}", value);
    }
}