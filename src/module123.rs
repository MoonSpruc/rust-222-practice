// Перше завдання
use std::fmt;

struct Point1 {
    x: i32,
    y: i32,
}

impl fmt::Display for Point1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.y)
    }
}
#[test]
fn test1() {
    let origin = Point1 { x: 0, y: 0 };

    // Заповнюємо прогалини
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");

    println!("Success! 1");
}

// ---------------------------------------------

// Друге завдання
// use std::str::FromStr;
// #[test]
// fn test2() {
//     let parsed: i32 = "5".parse().unwrap();
//     let turbo_parsed: i32 = "10".parse().unwrap(); // Явно вказуємо тип i32
//     let from_str: i32 = i32::from_str("20").unwrap(); // Також явно вказуємо тип i32
//     let sum = parsed + turbo_parsed + from_str;
//     assert_eq!(sum, 35);
//
//     println!("Success! 2");
// }

// ---------------------------------------------

// Третє завдання
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point3 {
    x: i32,
    y: i32,
}

impl FromStr for Point3 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .map(|x| x.trim())
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point3 { x: x_fromstr, y: y_fromstr })
    }
}
#[test]
fn test3() {
    // Заповнюємо пропуски двома способами
    let p = Point3::from_str("(3, 4)"); // Перший спосіб
    // Або другий спосіб:
    // let p = "(3, 4)".parse::<Point>(); // Другий спосіб

    assert_eq!(p.unwrap(), Point3 { x: 3, y: 4 });

    println!("Success! 3");
}
