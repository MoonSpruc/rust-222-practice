// https://www.hackerrank.com/challenges/beautiful-days-at-the-movies/problem?isFullScreen=true
/*
Умова:
1. Визначити кількість таких "красивих" днів у діапазоні [i...j].
*/

fn reverse_number(mut n: i32) -> i32 {
    let mut reversed = 0;
    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    reversed
}

fn beautifulDays(i: i32, j: i32, k: i32) -> i32 {
    let mut count = 0;

    for day in i..=j {
        let reversed_day = reverse_number(day);
        if (day - reversed_day).abs() % k == 0 {
            count += 1;
        }
    }

    count
}