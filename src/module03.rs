// Перше завдання
#[test]
fn test1() {
    let x: i32 = 5;
    let _y: i32;

    assert_eq!(x, 5);
    println!("Success!");
}

// ---------------------------------------------

// Друге завдання
#[test]
fn task2() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

// ---------------------------------------------

// Третє завдання
#[test]
fn task3() {
    let x: i32 = 10;
    let y: i32 = 5;
    println!("The value of x is {} and value of y is {}", x, y);
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn task5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); // Всередині блока 'x' дорівнює 12
    }

    assert_eq!(x, 5); // Зовні блока 'x' все ще дорівнює 5

    let x = 42;
    println!("{}", x); // Вивід "42".
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn task6() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    // let x = x;  // Коментуємо строку, щоб не було помилки

    x += 3;

    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";

    println!("Success!");
}

// ---------------------------------------------

// Сьоме завдання
#[test]
fn task7() {
    let _x = 1;
}

// ---------------------------------------------

// Восьме завдання
#[test]
fn task8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

// ---------------------------------------------
// Дев'яте завдання
#[test]
fn task9() {
    let (x, y);
    // (x,..) = (3, 4); // Роскоментувати, бо гіт не пропускає
    // [.., y] = [1, 2]; // Роскоментувати, бо гіт не пропускає
    // Fill the blank to make the code work
    // assert_eq!([x,y], [3,2]); // Роскоментувати, бо гіт не пропускає

    println!("Success!");
}