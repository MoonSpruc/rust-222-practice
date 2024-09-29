// Перше завдання
#[test]
fn test1() {
    let arr = [1, 2, 3, 4, 5];

    assert!(arr.len() == 5); // умова на довжину 5

    println!("Success! 1");
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    // Заповнюємо пропуск
    assert!(std::mem::size_of_val(&arr) == 12); // 3 chars * 4 bytes each = 12 bytes

    println!("Success! 2");
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    let list: [i32; 100] = [1; 1].into_iter().chain(std::iter::repeat(0)).take(100).collect::<Vec<i32>>().try_into().unwrap();

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success! 3");
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let _arr = ['1', '2', '3']; // Тепер всі елементи - char

    println!("Success! 4");
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[0]; // Змінюємо індекс на 0, щоб отримати 'a'

    assert!(ele == 'a');

    println!("Success! 5");
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `Get` повертає Option<T>
    let name0 = names.get(0).unwrap();

    // Змінюємо індекс на 1 для доступа до другого елемента
    let name1 = &names[1];

    println!("Success! 6");
}