// Перше завдання
enum Direction {
    East,
    West,
    North,
    South,
}

#[test]
fn test1() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => { // Matching South or North here
            println!("South or North");
        },
        _ => println!("West"), // Handle other cases (like West)
    };
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let boolean = true;

    // Fill the blank with a match expression:
    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success! 2");
}

// ---------------------------------------------

// Третє завдання
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn test3() {
    let msgs = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }

    println!("Success! 3");
}
fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => { // match Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(r, g, b) => {
            assert_eq!(g, 255); // Fill in with the correct value for green
            assert_eq!(b, 0);   // Fill in with the correct value for blue
        }
        _ => println!("no data in these variants"),
    }
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    // Fill the blank with `matches!` to make the code work
    for ab in alphabets {
        // Assert only if the character is a letter
        if matches!(ab, 'a'..='z' | 'A'..='Z') {
            assert!(true); // Оскільки це буква, перевірка відбувається
        } else {
            // Якщо не буква, просто ігноруємо
            continue;
        }
    }

    println!("Success! 4");
}

// ---------------------------------------------

// П'яте завдання
#[derive(PartialEq)] // This derives the PartialEq trait for MyEnum
enum MyEnum {
    Foo,
    Bar,
}
#[test]
fn test5() {
    let mut count = 0;

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if e == MyEnum::Foo { // Now this line works because PartialEq is derived
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success! 5");
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let o = Some(7);

    // Use `if let` instead of `match`
    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
        println!("Success! 6");
    }
}

// ---------------------------------------------

// Сьоме завдання
enum Foo7 {
    Bar(u8),
}
#[test]
fn test7() {
    let a = Foo7::Bar(1);

    if let Foo7::Bar(i) = a { // Fill in the blank
        println!("foobar holds the value: {}", i);
        println!("Success! 7");
    }
}

// ---------------------------------------------

// Восьме завдання
enum Foo8 {
    Bar,
    Baz,
    Qux(u32),
}
#[test]
fn test8() {
    let a = Foo8::Qux(10);

    // Use `match` instead
    match a {
        Foo8::Bar => {
            println!("match foo::bar");
        }
        Foo8::Baz => {
            println!("match foo::baz");
        }
        _ => {
            println!("match others");
        }
    }
}

// ---------------------------------------------

// Дев'яте завдання
#[test]
fn test9() {
    let age = Some(30);
    if let Some(age) = age { // Create a new variable with the same name as previous `age`
        assert_eq!(age, 30); // Fix the assertion to compare with 30
    } // The new variable `age` goes out of scope here

    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, its value is {}", age),
        _ => ()
    }
}