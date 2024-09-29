// Перше завдання
#[test]
fn test1() {
    let s: &str = "hello, world";

    println!("Success! 1");
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let s: Box<str> = "hello, world 2".into();
    greetings(s) // Передаємо Box<str>
}
fn greetings(s: Box<str>) {
    println!("{}", s)
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success! 3");
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let mut s = String::from("hello");
    s.push(',');                // Додаємо символ
    s.push_str(" world 4");        // Додаємо строку
    s += &"!".to_string();       // Додаємо строку через оператор +=

    println!("{}", s);
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let s = String::from("I like dogs");
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success! 5");
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Передаємо s2 як посилання
    assert_eq!(s3, "hello,world!");
    println!("{}", s3);
}

// ---------------------------------------------

// Сьоме завдання
#[test]
fn test7() {
    let s = "hello, world 7";
    greetings2(s.to_string());
}
fn greetings2(s: String) {
    println!("{}", s);
}

// ---------------------------------------------

// Восьме завдання
#[test]
fn test8() {
    let s = "hello, world".to_string();
    let s1: &str = &s;  // Використовуємо посилання на String

    println!("Success! 8");
}

// ---------------------------------------------

// Дев'яте завдання
#[test]
fn test9() {
    let byte_escape = "I'm writing Ru\x73\x74!";  // Додаємо \x74 для символа 't'
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

// ---------------------------------------------

// Десяте завдання
#[test]
fn test10() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}".replace(r"\x3F", "?").replace(r"\u{211D}", "ℝ"); // Замінюємо екрановані символи
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);


    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = r###"Hello, "##""###; // Використовуємо 3 # для екранування
    assert_eq!(long_delimiter, "Hello, \"##\""); // Перевірка на екранування лапок

    println!("Success! 10");
}

// ---------------------------------------------

// Одинадцяте завдання
#[test]
fn test11() {
    let s = String::from("hi,中国");

    // Виправляємо щоб h був першим символом
    let h = s.chars().nth(0).unwrap(); // Вилучаємо перший символ
    assert_eq!(h.to_string(), "h");    // Перевіряємо, що це "h"

    // Вилучаємо символ "中"
    let h1 = s.chars().nth(3).unwrap(); // Отримуємо 4-й символ
    assert_eq!(h1.to_string(), "中");    // Перевіряємо, що це "中"

    println!("Success! 11");
}

#[test]
fn test12() {
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}
