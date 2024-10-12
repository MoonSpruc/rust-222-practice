// https://www.hackerrank.com/challenges/the-birthday-bar/problem?isFullScreen=true
/*
Умова:
1. Треба знайти кількість відрізків масиву довжиною m, сума елементів яких дорівнює d.
*/

fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut count = 0;

    for i in 0..=(s.len() as i32 - m) {
        let segment_sum: i32 = s[i as usize..(i + m) as usize].iter().sum();
        if segment_sum == d {
            count += 1;
        }
    }

    count
}