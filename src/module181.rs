// Перше завдання
#[test]
fn test1() {
    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    print();
    print();

    // Тепер `color` можна знову позичити як незмінне посилання.
    let _reborrow = &color;

    println!("{}", color);
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let mut count = 0;

    let mut inc = || {
        let mut local_count = count; // Створюємо копію значення
        local_count += 1;
        println!("`count`: {}", local_count);
    };

    inc();
    inc();

    assert_eq!(count, 0);
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        take(movable.clone()); // Використовуємо `clone`, щоб не переміщати оригінал
    };

    consume();
    consume();
}
fn take<T>(_v: T) {}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {

}

// ---------------------------------------------

// П'яте завдання
fn fn_once<F>(func: F)
where
    F: Fn(usize) -> bool, // Використовуємо `Fn` для позичання по посиланню
{
    println!("{}", func(3));
    println!("{}", func(4));
}
#[test]
fn test5() {
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len()); // Тепер змінна `x` позичається
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec6(update_string);

    println!("{:?}", s);
}
fn exec6<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}

// ---------------------------------------------

// Сьоме завдання
#[test]
fn test7() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
fn apply<F>(f: F)
where
// The closure takes no input and returns nothing.
    F: FnOnce() {

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32
where
// The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

// ---------------------------------------------

// Восьме завдання
#[test]
fn test8() {
    let mut s = String::new();

    // Використовуємо `&mut s` для зміни `s` без переміщення
    let update_string = |str: &str| -> String { s.push_str(str); s.clone() }; // Повертаємо копію s

    exec8(update_string);
}
fn exec8<'a, F: FnMut(&'a str) -> String>(mut f: F) {
    f("hello");
}

// ---------------------------------------------

// Дев'яте завдання
#[test]
fn test9() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
fn call_me<F>(f: F)
where
    F: Fn(), // Це дозволяє `f` бути замиканням або функцією
{
    f(); // Викликаємо передану функцію або замикання
}

fn function() {
    println!("I'm a function!");
}

// ---------------------------------------------

// Десяте завдання
#[test]
fn test10() {
    let fn_plain = create_fn();
    println!("{}", fn_plain(1)); // Виведе 6
}
fn create_fn() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    // Це замикання захоплює `num` за значенням
    Box::new(move |x| x + num)
}

// ---------------------------------------------

// Одинадцяте завдання
#[test]
fn test11() {
    let f = factory(2);
    println!("{}", f(3)); // Виведе 8
}
fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1 {
        Box::new(move |y| y + num) // Повертаємо замикання в обгортці Box
    } else {
        Box::new(move |y| y + num) // Повертаємо замикання в обгортці Box
    }
}
