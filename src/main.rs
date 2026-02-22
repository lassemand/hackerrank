#![allow(non_snake_case)]
mod equal;
mod sherlock;
mod construct;
mod palindromes;
mod strings;
mod encryption;
mod bigger_is_greater;
mod build_a_string;
mod maximum_subarray_sum;
mod connected_cell_in_a_grid;
mod short_palindrome;
mod organizing_containers;
mod kaprekar_numbers;
mod kingdom_division;
mod fibonacci_modified;
mod abbrevation;
mod sam_and_substrings;
mod prime_xor;
mod diagonal_difference;
mod mini_max_sum;
mod divisible_sum_pairs;
mod bon_appetit;

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
