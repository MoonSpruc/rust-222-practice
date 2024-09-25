// Перше завдання
#[test]
fn test1() {
    let c1 = 'a';
    assert_eq!(c1.len_utf8(), 1); // 1 byte in UTF-8

    let c2 = '中';
    assert_eq!(c2.len_utf8(), 3); // 3 bytes in UTF-8

    println!("Success! 1");
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {

}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success! 3");
    }
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success! 4");
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, (2, 3)); // Порівнюємо v з кортежем (2, 3)

    implicitly_ret_unit(); // Виклик функції для виповненя її тіла

    println!("Success! 5");
}
fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// ---------------------------------------------

// Шосте завдання

use std::mem::size_of_val;
#[test]
fn test6() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0); // Змінюємо 4 на 0

    println!("Success! 6");
}