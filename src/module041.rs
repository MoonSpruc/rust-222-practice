// Перше завдання
#[test]
fn test1() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x as u32;

    let z = 10; // Type of z ?

    println!("Success!");
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
        let x = 5;
        assert_eq!("i32".to_string(), type_of(&x));

        println!("Success!");
}
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{},{}",v1,v2);
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}

// ---------------------------------------------

// Сьоме завдання
#[test]
fn test7() {
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// ---------------------------------------------

// Восьме завдання
#[test]
fn test8() {
    let a: f32 = 0.1;
    let b: f32 = 0.2;
    assert!((a + b) == 0.3);

    println!("Success!");
}

// ---------------------------------------------

// Дев'яте завдання
// fn test9() {}

// ---------------------------------------------

// Десяте завдання
// #[test]
use std::ops::{Range, RangeInclusive};

fn test10() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

// ---------------------------------------------

// Одинадцяте завдання
#[test]
fn test11() {
    // Integer addition
    assert!(1u32 + 2 == 3); // 1 + 2 = 3

    // Integer subtraction
    assert!(1i32 - 2 == -1); // 1 - 2 = -1
    assert!(1u8 - 2 == 255); // 1u8 - 2 переповнює й стає 255 (0xFF)

    assert!(3 * 50 == 150); // 3 * 50 = 150
    assert!(9.6 / 3.2 == 3.0); // 9.6 / 3.2 = 3.0

    assert!(24 % 5 == 4); // 24 % 5 = 4

    assert!(true && false == false); // true AND false = false
    assert!(true || false == true);   // true OR false = true
    assert!(!true == false);           // NOT true = false

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // 0011 AND 0101 = 0001
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);  // 0011 OR 0101 = 0111
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); // 0011 XOR 0101 = 0110
    println!("1 << 5 is {}", 1u32 << 5);                      // 1 << 5 = 32
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);           // 0x80 >> 2 = 0x20
}
