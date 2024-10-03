// Перше завдання
// use std::fmt;
//
// /* Define the Wrapper type */
// struct Wrapper(Vec<String>);
//
// // Display is an external trait
// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }
//
// #[test]
// fn test1() {
//     // Vec is an external type, so you cannot implement Display trait on Vec type
//     let w = Wrapper(vec![String::from("hello"), String::from("world")]);
//     println!("w = {}", w);
// }

// ---------------------------------------------

// Друге завдання
// struct Meters(u32);
//
// impl Meters {
//     // Реалізуємо метод pow для структури Meters
//     fn pow(&self, exp: u32) -> u32 {
//         self.0.pow(exp) // Викликаємо pow для поля, що містить u32
//     }
// }
// #[test]
// fn test2() {
//     let i: u32 = 2;
//     assert_eq!(i.pow(2), 4);
//
//     let n = Meters(i);
//     // Тепер можемо викликати наш метод pow
//     assert_eq!(n.pow(2), 4);
// }

// ---------------------------------------------

// Третє завдання
struct Years(i64);
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

// An age verification function that checks age in years, must be given a value of type Years.
fn old_enough_years(age: &Years) -> bool {
    age.0 >= 18
}

// An age verification function that checks age in days, must be given a value of type Days.
fn old_enough_days(age: &Days) -> bool {
    age.to_years().0 >= 18
}

#[test]
fn test3() {
    let age = Years(5);
    let age_days = age.to_days();

    println!("Old enough (years): {}", old_enough_years(&age));
    println!("Old enough (days): {}", old_enough_days(&age_days));
}

// ---------------------------------------------

// Четверте завдання
use std::ops::Add;
use std::fmt;

struct Meters(u32);

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There are still {} meters left", self.0)
    }
}

impl Add for Meters {
    type Output = Self;

    fn add(self, other: Meters) -> Self {
        Self(self.0 + other.0)
    }
}

fn calculate_distance(a: Meters, b: Meters) -> Meters {
    a + b // Використовуємо реалізований оператор Add
}

#[test]
fn test4() {
    let d = calculate_distance(Meters(10), Meters(20));
    assert_eq!(format!("{}", d), "There are still 30 meters left");
    println!("{}", d); // Додано вивід для перевірки
}

// ---------------------------------------------

// П'яте завдання
// enum VeryVerboseEnumOfThingsToDoWithNumbers {
//     Add,
//     Subtract,
// }
//
// /* Fill in the blank */
// type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
// #[test]
// fn test5() {
//     // We can refer to each variant via its alias, not its long and inconvenient
//     // name.
//     let x = Operations::Add;
// }

// ---------------------------------------------

// Шосте завдання
// enum VeryVerboseEnumOfThingsToDoWithNumbers {
//     Add,
//     Subtract,
// }
//
// impl VeryVerboseEnumOfThingsToDoWithNumbers {
//     fn run(&self, x: i32, y: i32) -> i32 {
//         match self {
//             VeryVerboseEnumOfThingsToDoWithNumbers::Add => x + y,
//             VeryVerboseEnumOfThingsToDoWithNumbers::Subtract => x - y,
//         }
//     }
// }
// #[test]
// fn test6() {
//     let operation_add = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
//     let operation_subtract = VeryVerboseEnumOfThingsToDoWithNumbers::Subtract;
//
//     let result_add = operation_add.run(5, 3);
//     let result_subtract = operation_subtract.run(5, 3);
//
//     println!("5 + 3 = {}", result_add);        // Виведе: 5 + 3 = 8
//     println!("5 - 3 = {}", result_subtract);   // Виведе: 5 - 3 = 2
// }

// ---------------------------------------------

// Сьоме завдання
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,        // Використовуємо Self для посилання на сам тип
            Self::Subtract => x - y,   // Використовуємо Self для посилання на сам тип
        }
    }
}
#[test]
fn test7() {
    let operation_add = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    let operation_subtract = VeryVerboseEnumOfThingsToDoWithNumbers::Subtract;

    let result_add = operation_add.run(5, 3);
    let result_subtract = operation_subtract.run(5, 3);

    println!("5 + 3 = {}", result_add);        // Виведе: 5 + 3 = 8
    println!("5 - 3 = {}", result_subtract);   // Виведе: 5 - 3 = 2
}

// ---------------------------------------------

// Восьме завдання
fn my_function<const N: usize>() -> [u32; N] {
    [123; N] // Створюємо масив з N елементами, кожен з яких дорівнює 123
}
#[test]
fn test8() {
    let arr: [u32; 5] = my_function::<5>(); // Вказуємо константну довжину масиву
    println!("{:?}", arr);
}

// ---------------------------------------------

// Дев'яте завдання
use std::fmt::Display;

fn foobar(thing: Box<dyn Display>) {
    println!("{}", thing); // Виводимо thing
}
#[test]
fn test9() {
    let my_string = String::from("Hello, world!");
    foobar(Box::new(my_string)); // Передаємо Box з рядком
}
