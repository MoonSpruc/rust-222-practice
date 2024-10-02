// Перше завдання
#[test]
fn test1() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // FIX the error in two ways
    let i3: i32 = 'a' as i32; // Виправлення для char в i32

    // FIX the error in two ways
    let s: String = 'a'.to_string(); // Виправлення для char в String

    println!("Success! 1");
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // FIX the error in two ways
    let i3: i32 = 'a' as i32; // Виправлення для char в i32

    // FIX the error in two ways
    let s: String = 'a'.to_string(); // Виправлення для char в String

    println!("Success! 2");
}

// ---------------------------------------------

// Третє завдання
use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::IoError(err) // Перетворення io::Error у CliError
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::ParseError(err) // Перетворення ParseIntError у CliError
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? автоматично перетворює io::Error у CliError
    let contents = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}
#[test]
fn test3() {
    match open_and_parse_file("file.txt") {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => match e {
            CliError::IoError(err) => eprintln!("I/O error: {}", err),
            CliError::ParseError(err) => eprintln!("Parse error: {}", err),
        },
    }
}

// ---------------------------------------------

// Четверте завдання
// use std::convert::TryInto;
// #[test]
// fn test4() {
//     let n: i16 = 256;
//
//     // Використання TryInto для перетворення з i16 в u8
//     let n: u8 = match n.try_into() {
//         Ok(n) => n,
//         Err(e) => {
//             println!("there is an error when converting: {:?}, but we catch it", e.to_string());
//             0 // Якщо сталася помилка, повертаємо 0
//         }
//     };
//
//     assert_eq!(n, 0); // В даному випадку n буде 0, оскільки 256 не поміститься в u8
//
//     println!("Success! 4");
// }

// ---------------------------------------------

// П'яте завдання
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    // Реалізація `try_from`
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}
#[test]
fn test5() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // Заповнення пропусків
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8))); // Очікуємо успішний результат
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(())); // Очікуємо помилку

    println!("Success! 5");
}
