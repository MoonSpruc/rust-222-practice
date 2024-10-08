// https://www.hackerrank.com/challenges/solve-me-first/problem?isFullScreen=true
/*
Умова:
1. Зчитування двох чисел із стандартного потоку введення та виведення їх суми (A+B)
у стандартний потік виведення.
*/

use std::io;

#[test]
fn main() {
    // variable declaration
    let mut num_str_1 = String::new();
    let mut num_str_2 = String::new();

    // read variables
    io::stdin().read_line(&mut num_str_1).ok().expect("read error");
    io::stdin().read_line(&mut num_str_2).ok().expect("read error");

    // parse integers
    let num_1: i32 = num_str_1.trim().parse().ok().expect("parse error");
    let num_2: i32 = num_str_2.trim().parse().ok().expect("parse error");

    // print the sum
    println!("{}", num_1 + num_2);
}