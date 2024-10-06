// https://www.hackerrank.com/challenges/kangaroo/problem?isFullScreen=true
/*
Умова:
1. Чи зустрінуться два кенгуру, що стартують з різних точок на числовій осі та рухаються з різною швидкістю.
*/

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    // Якщо один кенгуру позаду і стрибає повільніше, вони ніколи не зустрінуться
    if (v1 == v2 && x1 != x2) || (x1 < x2 && v1 <= v2) || (x1 > x2 && v1 >= v2) {
        return "NO".to_string();
    }

    // Якщо різниця позицій ділиться на різницю швидкостей без залишку, то вони зустрінуться
    if (x2 - x1) % (v1 - v2) == 0 {
        return "YES".to_string();
    }

    // Інакше - не зустрінуться
    "NO".to_string()
}

#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let x1 = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let v1 = first_multiple_input[1].trim().parse::<i32>().unwrap();
    let x2 = first_multiple_input[2].trim().parse::<i32>().unwrap();
    let v2 = first_multiple_input[3].trim().parse::<i32>().unwrap();

    let result = kangaroo(x1, v1, x2, v2);

    println!("{}", result);
}