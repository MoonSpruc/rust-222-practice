// Перше завдання
#[test]
fn test1() {
    let arr = [0; 10];
    for val in arr.iter() {
        println!("{}", val);
    }
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let mut v = Vec::new();
    for n in 0..100 { // Заповнюємо пропуск
        v.push(n);
    }

    assert_eq!(v.len(), 100);
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    let mut v1 = vec![1, 2].into_iter(); // Використовуємо `into_iter()` для створення ітератора

    assert_eq!(v1.next(), Some(1)); // Перше значення
    assert_eq!(v1.next(), Some(2)); // Друге значення
    assert_eq!(v1.next(), None);     // Третє значення, немає більше елементів
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let arr = vec![0; 10];
    for &i in &arr { // Використовуємо &arr, щоб ітерувати по посиланнях
        println!("{}", i);
    }

    println!("{:?}", arr); // Тепер arr все ще доступний
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() { // Заповнюємо пропуск
        *name = match *name {
            "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut(); // Заповнюємо пропуск 1

    if let Some(v) = values_iter.next() { // Заповнюємо пропуск 2
        *v = 0; // Заповнюємо пропуск 3
    }

    assert_eq!(values, vec![0, 2, 3]);
}

// ---------------------------------------------

// Сьоме завдання
// #[test]
// fn test7() {
//     let mut fib = fibonacci();
//     assert_eq!(fib.next(), Some(1)); // Перший виклик повертає 1
//     assert_eq!(fib.next(), Some(1)); // Другий виклик повертає 1
//     assert_eq!(fib.next(), Some(2)); // Третій виклик повертає 2
//     assert_eq!(fib.next(), Some(3)); // Четвертий виклик повертає 3
//     assert_eq!(fib.next(), Some(5)); // П'ятий виклик повертає 5
// }
// struct Fibonacci {
//     curr: u32,
//     next: u32,
// }
//
// // Implement `Iterator` for `Fibonacci`.
// impl Iterator for Fibonacci {
//     // We can refer to this type using Self::Item
//     type Item = u32; // Встановлюємо Item як u32
//
//     /* Implement next method */
//     fn next(&mut self) -> Option<Self::Item> {
//         // Повертаємо поточне число
//         let curr = self.curr;
//         // Оновлюємо значення curr і next
//         self.curr = self.next;
//         self.next = curr + self.next;
//
//         // Повертаємо curr
//         if curr == 0 {
//             Some(1) // Повертаємо 1 при першому виклику
//         } else {
//             Some(curr) // Для всіх наступних викликів повертаємо curr
//         }
//     }
// }
//
// // Returns a Fibonacci sequence generator
// fn fibonacci() -> Fibonacci {
//     Fibonacci { curr: 0, next: 1 }
// }

// ---------------------------------------------

// Восьме завдання
#[test]
fn test8() {
    let v1 = vec![1, 2, 3];

    let total: i32 = v1.iter().sum(); // Створюємо новий ітератор для суми

    assert_eq!(total, 6); // Заповнюємо пропуск

    println!("{:?}", v1); // Тепер v1 все ще доступний
}

// ---------------------------------------------

// Дев'яте завдання
// use std::collections::HashMap;
// #[test]
// fn test9() {
//     let names = [("sunface", 18), ("sunfei", 18)];
//     let folks: HashMap<_, _> = names.iter().cloned().collect(); // Використовуємо `iter().cloned()` для копіювання
//
//     println!("{:?}", folks);
//
//     let v1: Vec<i32> = vec![1, 2, 3];
//
//     // Явно вказуємо тип для collect
//     let v2: Vec<i32> = v1.iter().cloned().collect(); // Використовуємо `cloned()` для отримання значень
//
//     assert_eq!(v2, vec![1, 2, 3]);
// }

// ---------------------------------------------

// Десяте завдання
#[test]
fn test10() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // Заповнюємо пропуски
    let v2: Vec<_> = v1.iter().map(|&x| x + 1).collect(); // Використовуємо map для додавання 1 до кожного елемента

    assert_eq!(v2, vec![2, 3, 4]);
}

// ---------------------------------------------

// Одинадцяте завдання
use std::collections::HashMap;
#[test]
fn test11() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];

    // Заповнюємо пропуски
    let folks: HashMap<_, _> = names.iter().zip(ages.iter()).collect(); // Використовуємо zip для поєднання пар

    println!("{:?}", folks);
}

// ---------------------------------------------

// Дванадцяте завдання
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect() // Заповнюємо пропуски
}
#[test]
fn test12() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}