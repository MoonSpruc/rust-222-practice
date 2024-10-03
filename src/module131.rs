// Перше завдання
#[test]
fn test1() {
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");
}
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success! 1");
    } else {
        panic!("This is not lemonade!");
    }

    println!("Exercise Failed if printing out this line!");
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    assert_eq!("abc".as_bytes(), [97, 98, 99]); // Виправлено ASCII-коди

    let v = vec![1, 2, 3];
    // let ele = v[3]; // Ця лінія призведе до паніки, тому її видаляємо
    if let Some(ele) = v.get(3) {
        println!("Found element: {}", ele);
    } else {
        println!("Element not found!");
    }

    // Виправляємо переповнення та інші можливі паніки
    let v = production_rate_per_hour(2);
    println!("Production rate: {}", v);

    // Захищаємо від ділення на нуль
    divide(15, 0);

    println!("Success! 2");
}
fn divide(x: u8, y: u8) {
    if y == 0 {
        println!("Cannot divide by zero");
    } else {
        println!("{}", x / y);
    }
}

fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: u8 = 221;
    match speed {
        1..=4 => speed.wrapping_mul(cph) as f64,
        5..=8 => speed.wrapping_mul(cph) as f64 * 0.9,
        9..=10 => speed.wrapping_mul(cph) as f64 * 0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    // $ RUST_BACKTRACE=full cargo run
}
