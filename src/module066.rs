// Перше завдання
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// Исправленное C-подобное перечисление с использованием целых чисел
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}
#[test]
fn test1() {
    // Преобразуем варианты перечислений в целые числа для сравнения
    assert_eq!(Number1::One as u8, Number::One as u8); // Сравнение между Number1 и Number
    assert_eq!(Number1::One as u8, Number2::One as u8); // Сравнение между Number1 и Number2

    println!("Success! 1");
}

// ---------------------------------------------

// Друге завдання
enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn test2() {
    let msg1 = Message2::Move { x: 1, y: 2 }; // Создаем экземпляр с x = 1, y = 2
    let msg2 = Message2::Write(String::from("hello, world!")); // Создаем экземпляр с "hello, world!"

    println!("Success! 2");
}

// ---------------------------------------------

// Третє завдання
enum Message3 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn test3() {
    let msg = Message3::Move { x: 1, y: 2 };

    if let Message3::Move { x, y } = msg {
        assert_eq!(x, 1); // Проверяем, что x равно 1
        assert_eq!(y, 2); // Проверяем, что y равно 2 (если это нужно)
    } else {
        panic!("NEVER LET THIS RUN!");
    }

    println!("Success! 3");
}

// ---------------------------------------------

// Четверте завдання
use std::fmt;

enum Message4 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Реализация трейта Display для перечисления Message
impl fmt::Display for Message4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message4::Quit => write!(f, "Quit"),
            Message4::Move { x, y } => write!(f, "Move to x: {}, y: {}", x, y),
            Message4::Write(text) => write!(f, "Write: {}", text),
            Message4::ChangeColor(r, g, b) => write!(f, "Change Color to R: {}, G: {}, B: {}", r, g, b),
        }
    }
}
#[test]
fn test4() {
    let msgs: [Message4; 3] = [ // Указываем тип массива
        Message4::Quit,
        Message4::Move { x: 1, y: 3 },
        Message4::ChangeColor(255, 255, 0),
    ];

    for msg in msgs.iter() { // Используем .iter() для итерации
        show_message(msg) // Передаем ссылку на msg
    }
}
fn show_message(msg: &Message4) { // Изменяем тип параметра на ссылку
    println!("{}", msg);
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Заполняем пропуски в if let
    if let Some(n) = six {
        println!("{}", n);
        println!("Success! 5");
    } else {
        // Обрабатываем случай, когда six равно None
        println!("six is None, preventing panic!");
    }

    // Убираем panic!
    // panic!("NEVER LET THIS RUN!"); // Это можно удалить или оставить закомментированным
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1), // Заполняем пропуски для обработки Some
        None => None, // Обрабатываем None
    }
}

// ---------------------------------------------

// Шосте завдання
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        List::Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        List::Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // Match on the current instance of List
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // Instead take a reference to the tail
            List::Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            List::Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            List::Nil => {
                format!("Nil")
            },
        }
    }
}

#[test]
fn test6() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
