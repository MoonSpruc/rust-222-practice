// Перше завдання
#[test]
fn test1() {
    let _t0: (u8, i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success! 1");
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    println!("Success! 2");
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    let too_long_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    println!("too long array: {:?}", too_long_array);
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let tup = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let (x, z, y) = tup; // Розпакування кортежа

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success! 4");
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let (x, y, z);

    // Fill the blank
    (y, z, x) = (1, 2, 3); // Измените порядок переменных, чтобы соответствовать assert_eq!

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success! 5");
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let (x, y) = sum_multiply((2, 3)); // Передаем кортеж (2, 3)

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success! 6");
}
fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}