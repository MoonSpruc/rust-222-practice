// https://www.hackerrank.com/challenges/a-very-big-sum/problem?isFullScreen=true
/*
Умова:
1. Обчислити і вивести суму елементів масиву з огляду на те,
що деякі з цих цілих чисел можуть бути досить великими.
*/

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn aVeryBigSum(ar: &[i64]) -> i64 {
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ar: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    let result = aVeryBigSum(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}
