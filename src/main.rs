mod equal;
mod sherlock;
mod construct;
mod palindromes;
mod strings;
mod encryption;
mod bigger_is_greater;

use std::io::{self, Read};
use crate::equal::min_operations;

fn main() {
    // Fast input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let t: u8 = iter.next().unwrap().parse().unwrap();
    let mut output = String::new();

    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let mut a = Vec::with_capacity(n);

        for _ in 0..n {
            a.push(iter.next().unwrap().parse::<i16>().unwrap());
        }

        let result = min_operations(&a);
        output.push_str(&format!("{}\n", result));
    }

    print!("{}", output);
}
