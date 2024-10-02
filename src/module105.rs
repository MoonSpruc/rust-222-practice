// Перше завдання
#[test]
fn test1() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
             &number_1, &number_2,
             container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
// Структура Container, яка містить два цілі числа
struct Container(i32, i32);

// Використовуємо асоційовані типи для повторної реалізації трейту Contains
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

// Реалізація трейту Contains для структури Container
impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Повертає перше число
    fn first(&self) -> i32 { self.0 }

    // Повертає останнє число
    fn last(&self) -> i32 { self.1 }
}

// Функція, яка обчислює різницю між останнім і першим числом
fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
    container.last() - container.first()
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
               Point { x: 1, y: 3 });

    println!("Success! 2");
}
use std::ops::Sub;

#[derive(Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}

// Використання стандартних параметрів типу T для реалізації трейту Sub
impl<T> Sub for Point<T>
where
    T: Sub<Output = T>, // Обмеження: тип T повинен реалізувати Sub
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    let person = Human;

    // Виклик методу fly з трейту Pilot
    assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
    // Виклик методу fly з трейту Wizard
    assert_eq!(Wizard::fly(&person), "Up!");

    // Виклик методу fly з структури Human
    assert_eq!(person.fly(), "*waving arms furiously*");

    println!("Success! 3");
}
trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("This is your captain speaking.")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("Up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("*waving arms furiously*")
    }
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string(),
    };

    // Pass the student to the function
    println!("{}", comp_sci_student_greeting(&student));
}
trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

// Implement the necessary traits for CSStudent to make the code work
impl Person for CSStudent {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for CSStudent {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for CSStudent {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl CompSciStudent for CSStudent {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}
use std::fmt;

// DEFINE a newtype `Pretty` here
struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world 5")
    }
}
