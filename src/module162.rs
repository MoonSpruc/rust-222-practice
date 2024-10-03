// Перше завдання
// use std::fmt; // Імпортуємо модуль fmt
//
// // Додаємо реалізацію трейту fmt::Debug для структури
// struct Structure(i32);
//
// // Реалізуємо трейт fmt::Debug для Structure
// impl fmt::Debug for Structure {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Structure({})", self.0) // Виводимо значення в структурі
//     }
// }
// #[test]
// fn test1() {
//     // Types in std and Rust have implemented the fmt::Debug trait
//     println!("{} months in a year.", 12); // Використовуємо {} для виводу
//
//     println!("Now {:?} will print!", Structure(3)); // Використовуємо {:?} для виводу
// }

// ---------------------------------------------

// Друге завдання
// use std::fmt;
//
// // Оголошуємо структуру
// #[derive(Debug)] // Можна залишити derive Debug для відладки, якщо потрібно
// struct Person {
//     name: String,
//     age: u8,
// }
//
// // Реалізуємо трейт fmt::Display для Person
// impl fmt::Display for Person {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Person {{\n    name: \"{}\",\n    age: {},\n}}", self.name, self.age)
//     }
// }
// #[test]
// fn test2() {
//     let person = Person { name: "Sunface".to_string(), age: 18 };
//
//     // Використовуємо трейт Display для виводу
//     println!("{}", person); // Тепер це викликає реалізацію Display
// }

// ---------------------------------------------

// Третє завдання
// use std::fmt;
//
// // Оголошуємо структуру Structure
// struct Structure(i32);
//
// // Реалізуємо трейт fmt::Display для Structure
// impl fmt::Display for Structure {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0) // Виводимо тільки значення
//     }
// }
//
// // Реалізуємо трейт fmt::Debug для Structure
// impl fmt::Debug for Structure {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0) // Виводимо тільки значення
//     }
// }
//
// // Оголошуємо структуру Deep
// struct Deep(Structure);
//
// // Реалізуємо трейт fmt::Debug для Deep
// impl fmt::Debug for Deep {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0) // Виводимо тільки значення Structure
//     }
// }
// #[test]
// fn test3() {
//     // Тепер це буде виводити тільки значення
//     println!("Now {:?} will print!", Deep(Structure(7)));
// }

// ---------------------------------------------

// Четверте завдання
// use std::fmt;
//
// // Оголошуємо структуру Point2D
// struct Point2D {
//     x: f64,
//     y: f64,
// }
//
// // Реалізуємо трейт fmt::Display для Point2D
// impl fmt::Display for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Display: {} + {}i", self.x, self.y) // Формат для Display
//     }
// }
//
// // Реалізуємо трейт fmt::Debug для Point2D
// impl fmt::Debug for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y) // Формат для Debug
//     }
// }
//
// #[test]
// fn test4() {
//     let point = Point2D { x: 3.3, y: 7.2 };
//
//     // Перевіряємо формати
//     assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
//     assert_eq!(format!("{:?}", point), "Debug: Complex { real: 3.3, imag: 7.2 }");
//
//     println!("Success! 4");
// }

// ---------------------------------------------

// П'яте завдання
use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Отримуємо вектор
        let vec = &self.0;

        write!(f, "[")?; // Відкриваємо квадратні дужки

        // Проходимо по елементах вектора з їхніми індексами
        for (count, v) in vec.iter().enumerate() {
            // Для кожного елемента, крім першого, додаємо кому
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?; // Виводимо індекс та значення
        }

        // Закриваємо квадратні дужки та повертаємо результат
        write!(f, "]")
    }
}

#[test]
fn test5() {
    let v = List(vec![1, 2, 3]);
    assert_eq!(format!("{}", v), "[0: 1, 1: 2, 2: 3]");
    println!("Success! 5");
}
