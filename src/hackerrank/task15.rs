// https://www.hackerrank.com/challenges/day-of-the-programmer/problem?isFullScreen=true
/*
Умова:
1. Знайти дату 256-го дня року (День програміста) для будь-якого року в діапазоні від 1700 до 2700.
*/

fn day_of_programmer(year: i32) -> String {
    // Юліанський календар (1700-1917)
    if year >= 1700 && year <= 1917 {
        if year % 4 == 0 {
            return format!("12.09.{}", year);
        } else {
            return format!("13.09.{}", year);
        }
    }
    // Перехідний рік (1918)
    else if year == 1918 {
        return "26.09.1918".to_string();
    }
    // Григоріанський календар (1919 і далі)
    else {
        if (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0) {
            return format!("12.09.{}", year);
        } else {
            return format!("13.09.{}", year);
        }
    }
}