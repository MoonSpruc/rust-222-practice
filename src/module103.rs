// Перше завдання
#[test]
fn test1() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success! 1");
}
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}

impl Hello for Student {
    fn say_something(&self) -> String {
        String::from("I'm a good student")
    }
}

struct Teacher {}

impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }

    fn say_something(&self) -> String {
        String::from("I'm not a bad teacher")
    }
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let _one_second = Seconds(1);

    println!("One second looks like: {:?}", _one_second);
    let _this_is_true = (_one_second == _one_second);
    let _this_is_false = (_one_second > _one_second);

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}
// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// ADD some attributes to make the code work!
// DON'T modify other code!
#[derive(Debug, PartialEq, PartialOrd)] // Added these attributes
struct Seconds(i32);

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success! 3");
}
use std::ops::Mul;

// Implement fn multiply to make the code work.
fn multiply<T: Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    // DON'T modify the code below.
    // You need to derive some trait for FooBar to make it comparable.
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);

    println!("Success! 4");
}
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)] // Derive Debug for FooBar
struct FooBar;

#[derive(Debug)] // Derive Debug for BarFoo
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

// Implement Sub for Foo
impl ops::Sub<Bar> for Foo {
    type Output = BarFoo;

    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}

// Implement Sub for Bar
impl ops::Sub<Foo> for Bar {
    type Output = BarFoo;

    fn sub(self, _rhs: Foo) -> BarFoo {
        BarFoo
    }
}

// Derive PartialEq for FooBar to make it comparable
impl PartialEq for FooBar {
    fn eq(&self, _other: &FooBar) -> bool {
        true // Always return true, as they are the same type
    }
}

impl PartialEq for BarFoo {
    fn eq(&self, _other: &BarFoo) -> bool {
        true // Always return true, as they are the same type
    }
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(&post); // Pass a reference to `post`
    summary(&weibo); // Pass a reference to `weibo`

    println!("{:?}", post); // Now you can use `post` again
    println!("{:?}", weibo); // You can use `weibo` again
}
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

// Implement `fn summary` below.
fn summary<S: Summary>(item: &S) { // Change to accept a reference
    println!("{}", item.summarize());
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

// Returns a Box<dyn Animal> to use trait objects
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

// ---------------------------------------------

// Сьоме завдання
#[test]
fn test7() {
    assert_eq!(sum(1, 2), 3);
}
use std::ops::Add;

fn sum<T>(x: T, y: T) -> T
where
    T: Add<Output = T>, // Вказуємо, що T має реалізовувати трейт Add
{
    x + y
}

// ---------------------------------------------

// Восьме завдання
#[test]
fn test8() {
    let pair = Pair {
        x: Unit(1),
        y: Unit(3),
    };

    pair.cmp_display();
}
use std::fmt;

#[derive(Debug)] // Automatically derive Debug
struct Unit(i32);

impl PartialOrd for Unit { // Implement PartialOrd
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0) // Compare the inner values
    }
}

impl PartialEq for Unit { // Implement PartialEq
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 // Compare the inner values
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}

// ---------------------------------------------

// Дев'яте завдання
#[test]
fn test9() {
    example1();
    example2();

    println!("Success! 9");
}
use std::collections::HashMap;
use std::hash::Hash;
fn example1() {
struct Cacher<T>
where
T: Fn(u32) -> u32,
{
calculation: T,
values: HashMap<u32, u32>, // Хранить результаты для каждого аргумента
}

impl<T> Cacher<T>
where
T: Fn(u32) -> u32,
{
fn new(calculation: T) -> Cacher<T> {
Cacher {
calculation,
values: HashMap::new(),
}
}

fn value(&mut self, arg: u32) -> u32 {
// Проверяем, есть ли значение в кэше
if let Some(&v) = self.values.get(&arg) {
v
} else {
let v = (self.calculation)(arg);
self.values.insert(arg, v); // Сохраняем результат в кэше
v
}
}
}

let mut cacher = Cacher::new(|x| x + 1);
assert_eq!(cacher.value(10), 11); // 10 + 1 = 11
assert_eq!(cacher.value(15), 16); // 15 + 1 = 16
}

fn example2() {
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        values: HashMap<u32, u32>, // Хранить результаты для каждого аргумента
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                values: HashMap::new(),
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            // Проверяем, есть ли значение в кэше
            if let Some(&v) = self.values.get(&arg) {
                v
            } else {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v); // Сохраняем результат в кэше
                v
            }
        }
    }

    let mut cacher = Cacher::new(|x| x + 1);
    assert_eq!(cacher.value(20), 21); // 20 + 1 = 21
    assert_eq!(cacher.value(25), 26); // 25 + 1 = 26
}