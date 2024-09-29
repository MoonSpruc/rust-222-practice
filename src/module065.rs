// Перше завдання
struct Person1 {
    name: String,
    age: u8,
    hobby: String, // Додаємо поле hobby
}
#[test]
fn test1() {
    let age = 30;
    let p = Person1 {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"), // Ініціалізуємо поле hobby
    };

    println!("Success! 1");
}

// ---------------------------------------------

// Друге завдання
struct Unit;
trait SomeTrait {
    fn some_behavior(&self);
}

impl SomeTrait for Unit {
    fn some_behavior(&self) {
        println!("Unit is doing something!");
    }
}

fn do_something_with_unit(u: &dyn SomeTrait) {
    u.some_behavior(); // Виклик методу some_behavior
}
#[test]
fn test2() {
    let u = Unit;
    do_something_with_unit(&u); // Передаємо посилання на u

    println!("Success! 2");
}

// ---------------------------------------------

// Третє завдання
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[test]
fn test3() {
    let v = Point(0, 127, 255); // Заповнюємо параметри для Point
    check_color(Color(v.0, v.1, v.2)); // Перетворимо Point на Color

    println!("Success! 3");
}
fn check_color(p: Color) {
    let x = p.0; // Отримуємо значення з поля 0 структури Color
    let y = p.1; // Отримуємо значення з поля 1 структури Color
    let z = p.2; // Отримуємо значення з поля 2 структури Color

    assert_eq!(x, 0);
    assert_eq!(y, 127);
    assert_eq!(z, 255); // Перевіряємо значення поля 2
}

// ---------------------------------------------

// Четверте завдання
struct Person2 {
    name: String,
    age: u8,
}

#[test]
fn test4() {
    let age = 18;
    let mut p = Person2 { // Зробимо p змінним
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei"); // Виправляємо присвоєння імені

    println!("Success! 4");
}

// ---------------------------------------------

// П'яте завдання
struct Person3 {
    name: String,
    age: u8,
}

#[test]
fn test5() {
    println!("Success! 5");
}
fn build_person(name: String, age: u8) -> Person3 {
    Person3 {
        name, // Заповнюємо перепустку, використовуючи змінну name
        age,
    }
}

// ---------------------------------------------

// Шосте завдання
struct User6 {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[test]
fn test6() {
    let u1 = User6 {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success! 6");
}
fn set_email(u: User6) -> User6 {
    User6 {
        email: String::from("contact@im.dev"),
        active: u.active,           // Заповнюємо активність
        username: u.username,       // Заповнюємо ім'я користувача
        sign_in_count: u.sign_in_count, // Заповнюємо кількість входів
    }
}

// ---------------------------------------------

// Сьоме завдання
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[test]
fn test7() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Друк налагоджувальної інформації та привласнення значення
        height: 50,
    };

    dbg!(&rect1); // Друк налагоджувальної інформації

    println!("rect1: {:?}", rect1); // Друк налагоджувальної інформації в stdout
}

// ---------------------------------------------

// Восьме завдання
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
#[test]
fn test8() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = &f.name; // Використовуємо посилання на ім'я, щоб не переміщати його

    // ONLY modify this line
    println!("{}, {}, {:?}", _name, f.data, f); // Використовуємо _name замість f.name
}