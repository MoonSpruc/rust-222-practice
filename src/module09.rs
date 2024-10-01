// Перше завдання
#[test]
fn test1() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500);

    println!("Success! 1");
}
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Завершити метод area, який повертає площу прямокутника.
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let light = TrafficLight2 {
        color: "red".to_owned(),
    };
    // Don't take the ownership of `light` here.
    light.show_state();
    // ... Otherwise, there will be an error below
    println!("{:?}", light);
}
#[derive(Debug)]
struct TrafficLight2 {
    color: String,
}

impl TrafficLight2 {
    pub fn show_state(&self) { // Додаємо &self як параметр
        println!("the current state is {}", self.color); // Використовуємо self замість __
    }
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    let mut light = TrafficLight3 {
        color: "red".to_owned(),
    };

    light.show_state(); // Показати поточний стан
    light.change_state(); // Змінити стан
    light.show_state(); // Показати змінений стан

    println!("Success! 3");
}
struct TrafficLight3 {
    color: String,
}

impl TrafficLight3 {
    // Using `Self` to fill in the blank.
    pub fn show_state(&self) { // Використовуємо &self
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(&mut self) { // Використовуємо &mut self
        self.color = "green".to_string();
    }
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let light = TrafficLight4::new();
    assert_eq!(light.get_state(), "red");

    println!("Success! 4");
}
#[derive(Debug)]
struct TrafficLight4 {
    color: String,
}

impl TrafficLight4 {
    // 1. Implement an associated function `new`,
    // 2. It will return a TrafficLight contains color "red"
    // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
    pub fn new() -> Self { // Використовуємо Self замість TrafficLight
        Self {
            color: "red".to_owned(), // Повертаємо екземпляр із кольором "red"
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    println!("Success! 5");

}
struct Rectangle5 {
    width: u32,
    height: u32,
}

// Перший блок impl для методів, пов'язаних із обчисленнями
impl Rectangle5 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Другий блок impl для методів, пов'язаних із поведінкою
impl Rectangle5 {
    fn can_hold(&self, other: &Rectangle5) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}", c);
}
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    pub fn color(&self) -> &str {
        match self {
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
            TrafficLightColor::Green => "green",
        }
    }
}
