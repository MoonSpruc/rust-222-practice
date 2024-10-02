// Перше завдання
// use std::collections::HashMap;
// #[test]
// fn test1() {
//     let mut scores = HashMap::new();
//     scores.insert("Sunface", 98);
//     scores.insert("Daniel", 95);
//     scores.insert("Ashley", 69); // Змінено на i32
//     scores.insert("Katie", 58); // Змінено на i32
//
//     // Get returns an Option<&V>
//     let score = scores.get("Sunface");
//     assert_eq!(score, Some(&98)); // Порівнюємо з посиланням на значення
//
//     if scores.contains_key("Daniel") {
//         // Indexing returns a value V
//         let score = scores["Daniel"];
//         assert_eq!(score, 95); // Встановлюємо правильне значення
//         scores.remove("Daniel");
//     }
//
//     assert_eq!(scores.len(), 3); // Після видалення Daniel має залишитися 3 значення
//
//     for (name, score) in &scores { // Додаємо & для перебору по посиланню
//         println!("The score of {} is {}", name, score);
//     }
// }

// ---------------------------------------------

// Друге завдання
// use std::collections::HashMap;
//
// #[test]
// fn test2() {
//     let teams = [
//         ("Chinese Team", 100),
//         ("American Team", 10),
//         ("France Team", 50),
//     ];
//
//     let mut teams_map1 = HashMap::new();
//     for team in &teams {
//         teams_map1.insert(team.0, team.1);
//     }
//
//     // Спосіб 1: Використання for циклу для teams_map2
//     let mut teams_map2 = HashMap::new();
//     for &(name, score) in &teams {
//         teams_map2.insert(name, score);
//     }
//
//     assert_eq!(teams_map1, teams_map2);
//
//     println!("Success! 2");
// }

// ---------------------------------------------

// Третє завдання
// use std::collections::HashMap;
// #[test]
// fn test3() {
//     // Type inference lets us omit an explicit type signature (which
//     // would be `HashMap<&str, u8>` in this example).
//     let mut player_stats = HashMap::new();
//
//     // Insert a key only if it doesn't already exist
//     player_stats.entry("health").or_insert(100);
//
//     assert_eq!(player_stats["health"], 100); // Перший пропуск
//
//     // Insert a key using a function that provides a new value only if it
//     // doesn't already exist
//     player_stats.entry("health").or_insert_with(random_stat_buff);
//     assert_eq!(player_stats["health"], 100); // Другий пропуск
//
//     // Ensures a value is in the entry by inserting the default if empty, and returns
//     // a mutable reference to the value in the entry.
//     let health = player_stats.entry("health").or_insert(50);
//     assert_eq!(health, &100); // Третій пропуск
//     *health -= 50;
//     assert_eq!(*health, 50); // Четвертий пропуск
//
//     println!("Success! 3");
// }
// fn random_stat_buff() -> u8 {
//     // Could actually return some random value here - let's just return
//     // some fixed value for now
//     42
// }

// ---------------------------------------------

// Четверте завдання
// use std::collections::HashMap;
// #[test]
// fn test4() {
//     // Use a HashMap to store the vikings' health points.
//     let vikings = HashMap::from([
//         (Viking::new("Einar", "Norway"), 25),
//         (Viking::new("Olaf", "Denmark"), 24),
//         (Viking::new("Harald", "Iceland"), 12),
//     ]);
//
//     // Use derived implementation to print the status of the vikings.
//     for (viking, health) in &vikings {
//         println!("{:?} has {} hp", viking, health);
//     }
// }
// #[derive(Debug, Eq, Hash, PartialEq)] // Додано derive для Debug, Eq, Hash та PartialEq
// struct Viking {
//     name: String,
//     country: String,
// }
//
//
// impl Viking {
//     /// Creates a new Viking.
//     fn new(name: &str, country: &str) -> Viking {
//         Viking {
//             name: name.to_string(),
//             country: country.to_string(),
//         }
//     }
// }

// ---------------------------------------------

// П'яте завдання
use std::collections::HashMap;

#[test]
fn test5() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    let v2 = "hello".to_string();
    let mut m2 = HashMap::new();
    // Ownership moved here
    m2.insert(&v2, v1); // Використовуємо &v2 для передачі посилання

    assert_eq!(v2, "hello");

    println!("Success! 5");
}
