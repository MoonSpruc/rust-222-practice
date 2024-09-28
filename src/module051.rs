// Перше завдання
#[test]
fn test1() {
    let x = String::from("Hello world 1");
    let y = &x;
    println!("{}, {}", x, y);
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let s1 = String::from("Hello world 2");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    let s = give_ownership();
    println!("{}", s);
}
fn give_ownership() -> String {
    let s = String::from("Hello world 3");
    let s_bytes = s.clone().into_bytes(); // Клонуємо строку перед пертворенням
    let s = String::from_utf8(s_bytes).unwrap(); // Відновлюємо строку з байтів
    s
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let s = String::from("Hello World 4");

    print_str(&s);  // Передаємо посилання на строку

    println!("{}", s);  // Тепер строка ще доступна тут
}
fn print_str(s: &String)  {  // Приймаємо посилання на строку
    println!("{}", s);
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let x = (1, 2, (), "hello 5");
    let y = x; // Копіювання
    println!("{:?}, {:?}", x, y);
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success! 6");
}

// ---------------------------------------------

// Сьоме завдання
#[test]
fn test7() {
    let mut x = Box::new(5);

    let y = &mut *x;

    *y = 4;

    assert_eq!(*x, 4);

    println!("Success! 7");
}

// ---------------------------------------------

// Восьме завдання
#[test]
fn test8() {
    let t = (String::from("hello"), String::from("world"));

    let _s = &t.0;

    println!("{:?}, {:?}", _s, &t.1);
}

// ---------------------------------------------

// Дев'яте завдання
#[test]
fn test9() {
    let t = (String::from("hello"), String::from("world"));

    let (ref s1, ref s2) = t;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}