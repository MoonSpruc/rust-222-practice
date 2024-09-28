// Перше завдання
#[test]
fn test1() {
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success! 1");
}
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// ---------------------------------------------

// Друге завдання
#[test]
fn task2() {
    print();
}
fn print() {
    println!("Success! 2");
}

// ---------------------------------------------

// Третє завдання
#[test]
fn task3() {
    never_return();

    println!("Failed!");
}
fn never_return() -> ! {
    loop {

    }
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn task4() {
    println!("Success! 4");
}
fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => Some(42),
        _ => never_return_fn(),
    }
}
fn never_return_fn() -> ! {
    loop {
    }
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn task5() {
    let b = false;

    let _v = match b {
        true => 1,
        false => {
            println!("Success! 5");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}