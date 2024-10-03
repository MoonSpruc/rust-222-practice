// Перше завдання
#[test]
fn test1() {
    let v: &'static str = "hello"; // Варіант 1: Рядковий літерал
    need_static(v);

    println!("Success! 1")
}
fn need_static(r: &'static str) {
    assert_eq!(r, "hello");
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    unsafe {
        config = init();

        println!("{:?}", config);
    }
}
#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}

static mut config: Option<&'static mut Config> = None;

/* Make it work without changing the function signatures of `init` */
fn init() -> Option<&'static mut Config> {
    // Allocate the Config on the heap and leak it
    let config_box = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    // Use Box::leak to convert the Box into a &'static mut reference
    Some(Box::leak(config_box))
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    // Оголошення рядкового літерала з тривалістю 'static
    let static_string: &'static str = "I'm in read-only memory";
    println!("static_string: {}", static_string);

    {
        // Ви можете використовувати static_string в цьому блоці
        println!("static_string inside block: {}", static_string);
    }

    // Тепер ви можете використовувати його поза блоком
    println!("static_string reference remains alive: {}", static_string);
}

// ---------------------------------------------

// Четверте завдання
static NUM: i32 = 18; // Константа з тривалістю 'static

// Функція, що повертає посилання на NUM з прив'язкою до тривалості аргументу
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM // Повертаємо посилання на константу NUM
}
#[test]
fn test4() {
    {
        // Створюємо ціле число для використання в coerce_static:
        let lifetime_num = 9;

        // Приводимо NUM до тривалості lifetime_num:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    // NUM залишається доступним, навіть коли lifetime_num виходить із області видимості
    println!("NUM: {} stays accessible!", NUM);
}

// ---------------------------------------------

// П'яте завдання
use std::fmt::Debug;

fn print_it<T: Debug + 'static>(input: T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it1(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it2<T: Debug>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}
#[test]
fn test5() {
    // i є власником і не містить жодних посилань, отже, він має 'static:
    let i = 5;
    print_it(i); // Це працює

    // Ой, &i має лише тривалість, визначену областю видимості main(), тому це не 'static:
    // print_it(&i); // Цей виклик не працює

    // print_it1(&i); // Цей виклик також не працює, тому що &i не 'static

    // Але цей працює!
    print_it2(&i); // Це працює, оскільки передається посилання на `i`
}

// ---------------------------------------------

// Шосте завдання
use std::fmt::Display;
#[test]
fn test6() {
    let mut string = "First".to_owned();

    string.push_str(string.to_uppercase().as_str());
    print_a(&string);
    print_b(&string);
    // print_c(&string); // Помилка компіляції
    // print_d(&string); // Помилка компіляції
    print_e(&string);
    print_f(&string);
    // print_g(&string); // Помилка компіляції
}
fn print_a<T: Display + 'static>(t: &T) {
    println!("{}", t);
}

fn print_b<T>(t: &T)
where
    T: Display + 'static,
{
    println!("{}", t);
}

// Змінюємо тип, щоб не вимагати 'static
fn print_c(t: &dyn Display) {
    println!("{}", t)
}

// Змінюємо тип, щоб не вимагати 'static
fn print_d(t: &impl Display) {
    println!("{}", t)
}

// Ця функція вже вірна
fn print_e(t: &(dyn Display + 'static)) {
    println!("{}", t)
}

// Ця функція вже вірна
fn print_f(t: &(impl Display + 'static)) {
    println!("{}", t)
}

// Змінюємо тип, щоб не вимагати 'static
fn print_g(t: &String) {
    println!("{}", t);
}
