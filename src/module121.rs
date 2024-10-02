// Перше завдання
// #[test]
// fn test1() {
//     let decimal = 97.123_f32;
//
//     // Виправлення: заповнюємо пропуск типом u8
//     let integer: u8 = decimal as u8;
//
//     let c1: char = decimal as u8 as char;
//     let c2 = integer as char;
//
//     assert_eq!(integer, 'b' as u8);
//
//
//     println!("Success! 1");
// }

// ---------------------------------------------

// Друге завдання
// #![allow(overflowing_literals)]
// #[test]
// fn test2() {
//     assert_eq!(u8::MAX, 255);
//     // The max of `u8` is 255 as shown above.
//     // so the below code will cause an overflow error: literal out of range for `u8`.
//     // PLEASE looking for clues within compile errors to FIX it.
//     // DON'T modify any code in main.
//     let v = 1000 as u8;
//
//     println!("Success! 2");
// }

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    assert_eq!(1000 as u16, 1000); // 1000 вміщується в u16

    assert_eq!(1000 % 256, 232); // Залишок від ділення 1000 на 256 дорівнює 232

    // Для додатних чисел це те ж саме, що й модуль
    println!("1000 mod 256 is : {}", 1000 % 256);

    assert_eq!(-1_i8 as u8, 255); // -1 у i8 стане 255 у u8, оскільки це переповнення

    // Оскільки Rust 1.45, ключове слово `as` виконує *наповнювальний каст*
    // при приведенні з плаваючої точки до цілого. Якщо значення плаваючої точки перевищує
    // верхню межу або менше нижньої межі, повернене значення буде дорівнювати межі,
    // що була перетворена.
    assert_eq!(300.1_f32 as u8, 255); // 300.1 > 255, отже повертає 255
    assert_eq!(-100.1_f32 as u8, 0);   // -100.1 < 0, отже повертає 0

    // Ця поведінка має невеликий витратний час виконання,
    // але її можна уникнути за допомогою небезпечних методів.
    // Проте результати можуть переповнитися і
    // повернути **несумісні значення**. Використовуйте ці методи обережно:
    unsafe {
        // 300.0 у u8 - це 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 у u8 - це 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan у u8 - це 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize; // Перетворюємо p1 на адресу (usize)
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // p2 вказує на 2-й елемент в values
    unsafe {
        // Додаємо 1 до другого елемента
        *p2 += 1; // Використовуємо p2 для зміни значення
    }

    assert_eq!(values[1], 3);

    println!("Success! 4");
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let arr: [u64; 13] = [0; 13];
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64; 13] = &arr; // Зазначаємо розмір масиву
    let b = a as *const [u8; 104]; // 104 = 8 * 13
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 104) // Перевіряємо розмір
    }

    println!("Success! 5");
}
