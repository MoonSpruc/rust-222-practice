// https://www.hackerrank.com/challenges/drawing-book/problem?isFullScreen=true
/*
Умова:
1. Реалізувати функцію pageCount, яка обчислює мінімальну кількість перегортань сторінок,
необхідних, щоб дійти до заданої сторінки p у книзі з n сторінками.
*/

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'pageCount' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER p
 */

fn pageCount(n: i32, p: i32) -> i32 {
    let turns_from_front = p / 2;
    let turns_from_back = n / 2 - p / 2;
    return turns_from_front.min(turns_from_back);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = pageCount(n, p);

    writeln!(&mut fptr, "{}", result).ok();
}
