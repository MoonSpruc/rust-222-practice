// Перше завдання
#[test]
fn test1() {
    let i = 3; // `i` має найтривалішу тривалість життя. ────────────────────┐
    {
        let borrow1 = &i; // `borrow1` починає свою тривалість життя. ──┐ │
        //                                                               │ │
        println!("borrow1: {}", borrow1); //                             │ │
    } // `borrow1` закінчує свою тривалість життя. ─────────────────────┘ │
    {
        let borrow2 = &i; // `borrow2` починає свою тривалість життя. ──┐ │
        println!("borrow2: {}", borrow2);
    } // `borrow2` закінчує свою тривалість життя. ─────────────────────┘ │
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let x = 5; // `x` тепер живе до кінця функції
    {
        let r; // ---------+-- 'a
        { //          |
            r = &x; //  |       |
        } //          |        |
        println!("r: {}", r); //  |
    } // ---------+
}

// ---------------------------------------------

// Третє завдання
// Додаємо анотації тривалості життя до функції longest
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// #[test]
// fn test3() {
//     let string1 = String::from("long string");
//     let string2 = String::from("short");
//
//     let result = longest(&string1, &string2);
//     println!("The longest string is: {}", result);
// }

// ---------------------------------------------

// Четверте завдання
fn valid_output() -> String {
    String::from("foo") // Повертаємо новий String
}
#[test]
fn test4() {
    let result = valid_output();
    println!("{}", result); // Використовуємо результат
}

// ---------------------------------------------

// П'яте завдання
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}
fn successful_return<'a>() -> i32 {
    let x = 12; // `x` живе в межах функції
    x // Повертаємо значення, а не посилання
}
#[test]
fn test5() {
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);

    let result = successful_return();
    println!("Result is {}", result); // Використовуємо результат
}

// ---------------------------------------------

// Шосте завдання
// #[derive(Debug)]
// struct Borrowed<'a>(&'a i32); // Анотація тривалості життя для Borrowed
//
// #[derive(Debug)]
// struct NamedBorrowed<'a> { // Анотація тривалості життя для NamedBorrowed
//     x: &'a i32,
//     y: &'a i32,
// }
//
// #[derive(Debug)]
// enum Either<'a> { // Анотація тривалості життя для Either
//     Num(i32),
//     Ref(&'a i32),
// }
// #[test]
// fn test6() {
//     let x = 18;
//     let y = 15;
//
//     let single = Borrowed(&x);
//     let double = NamedBorrowed { x: &x, y: &y };
//     let reference = Either::Ref(&x);
//     let number    = Either::Num(y);
//
//     println!("x is borrowed in {:?}", single);
//     println!("x and y are borrowed in {:?}", double);
//     println!("x is borrowed in {:?}", reference);
//     println!("y is *not* borrowed in {:?}", number);
// }

// ---------------------------------------------

// Сьоме завдання
// #[derive(Debug)]
// struct NoCopyType {}
//
// #[derive(Debug)]
// struct Example<'a> { // Видалено параметр тривалості життя 'b
//     a: &'a u32,
//     b: Box<NoCopyType>, // Зберігаємо сам об'єкт NoCopyType в Box
// }
//
// #[test]
// fn test7() {
//     let var_a = 35;
//     let example: Example<'_>; // Вказуємо лише одну анотацію тривалості життя
//
//     {
//         let var_b = NoCopyType {};
//
//         // Тепер зберігаємо сам var_b в Box
//         example = Example { a: &var_a, b: Box::new(var_b) };
//     }
//
//     println!("(Success! 7) {:?}", example);
// }

// ---------------------------------------------

// Восьме завдання
#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType,
}

/* Fix function signature */
fn fix_me<'a, 'b>(foo: &'a Example<'_, 'b>) -> &'b NoCopyType {
    foo.b
}
#[test]
fn test8() {
    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };

    let result = fix_me(&example);
    println!("Success! 8 Reference to b: {:?}", result);
}

// ---------------------------------------------

// Дев'яте завдання
struct ImportantExcerpt<'a> {
    part: &'a str, // Додаємо параметр тривалості життя 'a до поля part
}

impl<'a> ImportantExcerpt<'a> { // Додаємо параметр тривалості життя 'a до імплементації
    fn level(&self) -> i32 { // Можемо використовувати &self без явного зазначення тривалості
        3
    }
}
#[test]
fn test9() {
    let text = String::from("This is an important excerpt.");
    let excerpt = ImportantExcerpt {
        part: &text, // Беремо посилання на текст
    };

    println!("Level: {}", excerpt.level());
}

// ---------------------------------------------

// Десяте завдання
fn input(x: &i32) { // Видалено 'a
    println!("`annotated_input`: {}", x);
}

fn pass(x: &i32) -> &i32 { x } // Видалено 'a

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // Додано 'a для тривалості життя
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Owner(i32);

impl Owner {
    // Видалено 'a
    fn add_one(&mut self) {
        self.0 += 1;
    }

    // Видалено 'a
    fn print(&self) {
        println!("`print`: {}", self.0);
    }
}

struct Person<'a> {
    age: u8,
    name: &'a str,
}

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}
#[test]
fn test10() {

}
