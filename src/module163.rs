// Перше завдання
// #[test]
// fn test1() {
//     println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
//     assert_eq!(format!("{1}{0}", 1, 2), "21"); // Перший пропуск
//     assert_eq!(format!("{0}{1}{0}", 1, 2), "2112"); // Другий пропуск
//     println!("Success! 1");
// }

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    println!("{argument}", argument = "test"); // => "test"

    /* Fill in the blanks */
    assert_eq!(format!("{name}{}", 1, name = "2"), "21"); // Перше заповнення
    assert_eq!(format!("{a} {c} {b}", a = "a", b = 'b', c = 3), "a 3 b"); // Друге заповнення

    /* Fix the error */
    println!("{0} {abc}", 2, abc = "def"); // Виправлення помилки

    println!("Success! 2");
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    // The following two are padding with 5 spaces
    println!("Hello {:5}!", "x"); // =>  "Hello x    !"
    println!("Hello {:1$}!", "x", 5); // =>  "Hello x    !"

    /* Fill in the blanks */
    assert_eq!(format!("Hello {:1$}!", "x", 5), "Hello x    !"); // Перше заповнення
    assert_eq!(format!("Hello {x:width$}!", x = "x", width = 5), "Hello x    !"); // Друге заповнення

    println!("Success! 3");
}

// ---------------------------------------------

// Четверте завдання
// #[test]
// fn test4() {
//     // Left align
//     println!("Hello {:<5}!", "x"); // => Hello x    !
//     // Right align
//     assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!"); // Правильний варіант
//     // Center align
//     assert_eq!(format!("Hello {:^7}!", "x"), "Hello   x  !"); // Центрування
//
//     // Left align, pad with '&'
//     assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&!"); // Вирівнювання з '&'
//
//     println!("Success! 4");
// }

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    println!("Hello {:5}!", 5); // => Hello     5!
    println!("Hello {:+}!", 5); // =>  Hello +5!
    println!("Hello {:05}!", 5); // => Hello 00005!
    println!("Hello {:05}!", -5); // => Hello -0005!

    /* Заповнити пропуск */
    assert!(format!("{number:0>width$}", number=1, width=6) == "000001");

    println!("Success! 5");
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let v = 3.1415926;

    println!("{:.1$}", v, 4); // same as {:.4} => 3.1416

    assert_eq!(format!("{:.2}", v), "3.14");
    assert_eq!(format!("{:+.2}", v), "+3.14");
    assert_eq!(format!("{:.0}", v), "3");

    println!("Success! 6");
}

// ---------------------------------------------

// Сьоме завдання
#[test]
fn test7() {

}

// ---------------------------------------------

// Восьме завдання
#[test]
fn test8() {
    let v = 3.1415926;

    println!("{:.1$}", v, 4); // same as {:.4} => 3.1416

    assert_eq!(format!("{:.2}", v), "3.14");       // Форматування до 2 знаків після коми
    assert_eq!(format!("{:+.2}", v), "+3.14");     // Форматування до 2 знаків з додатнім знаком
    assert_eq!(format!("{:.0}", v), "3");          // Форматування без десяткових знаків

    println!("Success! 8");
}

// ---------------------------------------------

// Дев'яте завдання
fn get_person() -> String {
    String::from("sunface")
}

fn get_format() -> (usize, usize) {
    (4, 1)
}
#[test]
fn test9() {
    let person = get_person();
    println!("Hello, {person}!");

    let (width, precision) = get_format();
    let scores = [("sunface", 99.12), ("jack", 60.34)];

    /* Make it print:
    sunface: 99.1
    jack: 60.3
    */
    for (name, score) in scores {
        println!("{name}: {:.1$}", score, precision);
    }
}
