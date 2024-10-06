// https://www.hackerrank.com/challenges/time-conversion/problem?isFullScreen=true
/*
Умова:
1. Написати функцію timeConversion, яка конвертує час у 12-годинному форматі (AM/PM) у 24-годинний формат.
*/

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    // Split the input string into time part and period (AM/PM)
    let period = &s[s.len()-2..]; // AM or PM
    let time_part = &s[..s.len()-2]; // Time without AM/PM

    // Split the time part into hours, minutes, and seconds
    let mut time_parts: Vec<&str> = time_part.split(':').collect();
    let mut hour: i32 = time_parts[0].parse().unwrap();

    if period == "AM" && hour == 12 {
        // Special case for 12:XX:XX AM -> 00:XX:XX
        hour = 0;
    } else if period == "PM" && hour != 12 {
        // Add 12 to the hour if it's PM and not 12 PM
        hour += 12;
    }

    // Format the hour back as a 2-digit number and reconstruct the time string
    format!("{:02}:{}:{}", hour, time_parts[1], time_parts[2])
}

#[test]
fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
