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
// fn test10() {}

// ---------------------------------------------

// Одинадцяте завдання
// fn test11() {}
