// Перше завдання
#[test]
fn test1() {
    let arrays: [&Array<i32, 3>; 2] = [
        &Array {
            data: [1, 2, 3],
        },
        &Array {
            data: [4, 5, 6],
        },
    ];

    let arrays_float: [&Array<f64, 3>; 1] = [
        &Array {
            data: [1.0, 2.0, 3.0],
        },
    ];

    println!("Success! 1");
}
struct Array<T, const N: usize> {
    data: [T; N],
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let arr = [1, 2, 3];
    print_array(&arr); // Передаємо посилання на масив

    let arr = ["hello", "world 2"];
    print_array(&arr); // Передаємо посилання на масив
}
fn print_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}

// ---------------------------------------------
#![allow(incomplete_features)] // Помилка
#![feature(generic_const_exprs)] // Помилка

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

fn main() {
    check_size([0u8; 767]); // Масив із 767 елементів типу u8
    check_size([0i32; 191]); // Масив із 191 елемента типу i32
    check_size(["hello你好"; 5]); // Розмір &str - 5 рядків
    check_size([(); 5].map(|_| "hello你好".to_string())); // Розмір String - 5 рядків
    check_size(['中'; 4]); // Розмір char - 4 символи

    println!("Success! 3");
}