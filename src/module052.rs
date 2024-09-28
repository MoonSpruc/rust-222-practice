// Перше завдання
#[test]
fn test1() {
    let x = 5;
    let p = &x;

    println!("the memory address of x is {:p}", p);
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let x = 5;
    let y = &x;

    assert_eq!(5, *y);

    println!("Success! 2");
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success! 3");
}
fn borrow_object(s: &String) {}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success! 4 String is: {}", s);
}
fn push_str(s: &mut String) {
    s.push_str("world");
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let mut s = String::from("hello, ");

    let p = &mut s;

    p.push_str("world");

    println!("Success! 5 String is: {}", s);
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let c = '中';

    let r1 = &c;
    let r2 = &c;

    assert_eq!(*r1, *r2);

    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success! 6");
}
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

// ---------------------------------------------

// Сьоме завдання
#[test]
fn test7() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;

    println!("{}", r1);

    println!("Success! 7");
}

// ---------------------------------------------

// Восьме завдання
#[test]
fn test8() {
    let mut s = String::from("hello, ");

    borrow_object2(&mut s);

    println!("Success! 8");
}
fn borrow_object2(s: &mut String) {}

// ---------------------------------------------

// Дев'яте завдання
#[test]
fn test9() {
    let mut s = String::from("hello, ");

    borrow_object3(&s);

    s.push_str("world");

    println!("Success! 9 String is: {}", s);
}
fn borrow_object3(s: &String) {}

// ---------------------------------------------

// Десяте завдання
#[test]
fn test10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world 10");
    // let r2 = &mut s;
    // r2.push_str("!");

    println!("{}", r1);
}

// ---------------------------------------------

// Одинадцяте завдання
#[test]
fn test11() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // Спробуємо використовувати r1 та r2 одночасно
    println!("{}, {}", r1, r2); // Це викличе помилку компіляції

    // Треба використати одне з посилань, щоб викликати помилку
    // println!("{}", r1);
    // println!("{}", r2);
}
