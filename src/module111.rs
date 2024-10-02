// Перше завдання
#[test]
// FILL in the blanks and FIX errors
// 1. Don't use `to_string()`
// 2. Don't add/remove any code line
fn test1() {
    let mut s: String = String::from("hello, "); // Ініціалізація змінної s
    s.push_str("world"); // Використання push_str без to_string()
    s.push('!'); // Заповнення пропуску

    move_ownership(&s); // Передача посилання на s

    assert_eq!(s, "hello, world!"); // Тепер це працює, оскільки s все ще доступна

    println!("Success! 1");
}

fn move_ownership(s: &String) { // Зміна параметра на посилання
    println!("ownership of \"{}\" is moved here!", s)
}


// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let mut s = String::from("hello, world");

    let slice1: &str = &s; // In two ways
    assert_eq!(slice1, "hello, world");

    let slice2 = &s[0..5]; // Слайс "hello"
    assert_eq!(slice2, "hello");

    let slice3: &mut String = &mut s; // Змінна, на яку можна викликати push
    slice3.push('!'); // Додаємо '!' до s
    assert_eq!(slice3, "hello, world!");

    println!("Success! 2");
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    // Question: how many heap allocations are happening here?
    // Your answer: 2


    // Create a String type based on &str
    // The type of string literals is &str
    let s: String = String::from("hello, world!");

    // Create a slice point to String s
    let slice: &str = &s;

    // Create a String type based on the recently created slice
    let s: String = slice.to_string();

    assert_eq!(s, "hello, world!");

    println!("Success! 3");
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1]; // Виправлено: беремо слайс з 0 до 1, щоб отримати "h"
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10]; // Виправлено: беремо слайс з 7 до 10, щоб отримати "世"
    assert_eq!(slice2, "世");

    // Iterate through all chars in s
    for (i, c) in s.chars().enumerate() { // Заповнено пропуск: використання chars() для ітерації
        if i == 7 {
            assert_eq!(c, '世') // Правильно: символ '世' знаходиться на позиції 7
        }
    }

    println!("Success! 4");
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let mut s = String::new();
    s.push_str("hello"); // Заповнено пропуск: ініціалізація рядка

    // Some bytes, in a vector
    let v = vec![104, 101, 108, 108, 111];

    // Turn a byte's vector into a String
    let s1 = String::from_utf8(v).unwrap(); // Заповнено пропуск: перетворення вектора байтів у String

    assert_eq!(s, s1);

    println!("Success! 5");
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let mut s = String::with_capacity(25); // Встановлюємо початкову місткість

    println!("{}", s.capacity()); // Друкуємо місткість (25)

    for _ in 0..2 {
        s.push_str("hello"); // Додаємо "hello"
        println!("{}", s.capacity()); // Друкуємо місткість (залишається 25)
    }

    println!("Success! 6");
}

// ---------------------------------------------

// Сьоме завдання
use std::mem;
#[test]
fn test7() {
    let story = String::from("Rust By Practice");

    // Prevent automatically dropping of the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr(); // Заповнено пропуск: отримуємо мутабельний вказівник на дані
    let len = story.len(); // Заповнено пропуск: отримуємо довжину
    let capacity = story.capacity(); // Заповнено пропуск: отримуємо місткість

    assert_eq!(16, len);

    // We can rebuild a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success! 7");
}
