// Перше завдання
#[test]
fn test1() {
    let duck = Duck1; // Створюємо качку
    duck.swim();      // Викликаємо метод swim для Duck

    let bird = hatch_a_bird(2);
    assert_eq!(bird.quack(), "duck duck"); // Перевіряємо що quack працює як очікувано

    let bird = hatch_a_bird(1);
    assert_eq!(bird.quack(), "swan swan");

    println!("Success! 1");
}
trait Bird1 {
    fn quack(&self) -> String;
}

struct Duck1;
impl Duck1 {
    fn swim(&self) {
        println!("Look, the duck is swimming");
    }
}
struct Swan1;
impl Swan1 {
    fn fly(&self) {
        println!("Look, the duck... oh sorry, the swan is flying");
    }
}

impl Bird1 for Duck1 {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird1 for Swan1 {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}
// Реалізуємо функцію hatch_a_bird
fn hatch_a_bird(n: i32) -> Box<dyn Bird1> {
    if n == 2 {
        Box::new(Duck1)
    } else {
        Box::new(Swan1)
    }
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    // Заповнюємо місце для створення вектора птахів
    let birds: Vec<Box<dyn Bird2>> = vec![Box::new(Duck2), Box::new(Swan2)];

    for bird in birds {
        bird.quack();
        // Коли качки та лебеді перетворюються на птахів, вони всі забули, як літати, лише пам'ятають, як квакають.
        // Код нижче викличе помилку.
        // bird.fly(); // Цей рядок викликає помилку, бо метод fly недоступний через трейт Bird
    }
}
trait Bird2 {
    fn quack(&self);
}

struct Duck2;
impl Duck2 {
    fn fly(&self) {
        println!("Look, the duck is flying");
    }
}

struct Swan2;
impl Swan2 {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying");
    }
}

impl Bird2 for Duck2 {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird2 for Swan2 {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    let x = 1.1f64;
    let y = 8u8;

    // Draw x.
    draw_with_box(Box::new(x)); // Заповнюємо пропуск для x

    // Draw y.
    draw_with_ref(&y); // Заповнюємо пропуск для y

    println!("Success! 3");
}
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}
fn draw_with_box(x: Box<dyn Draw>) {
    println!("{}", x.draw()); // Додаємо виведення результату
}

fn draw_with_ref(x: &dyn Draw) {
    println!("{}", x.draw()); // Додаємо виведення результату
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success! 4");
}
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// Реалізація статичної диспетчеризації
fn static_dispatch<T: Foo>(value: T) {
    println!("{}", value.method());
}

// Реалізація динамічної диспетчеризації
fn dynamic_dispatch(value: &dyn Foo) {
    println!("{}", value.method());
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));

    println!("Success! 5");
}
trait MyTrait {
    fn f(&self) -> Box<dyn MyTrait>; // Тип повернення вже Box<dyn MyTrait>
}

impl MyTrait for u32 {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(42) } // Повертаємо Box з u32
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) } // Повертаємо Box з String
}

fn my_function(x: Box<dyn MyTrait>) {
    x.f(); // Виклик методу f
}
