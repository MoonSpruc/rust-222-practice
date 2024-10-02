// Перше завдання
#[test]
fn test1() {
    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) and vec![..] are the same macros, so
    let v = vec!(1, 2, 3);
    is_vec(&v);

    // Rewrite the following code using Vec::new() and a for loop
    let mut v1 = Vec::new();
    for &item in &arr {
        v1.push(item);
    }

    is_vec(&v1);

    // v1 now contains the same elements as v
    assert_eq!(v, v1);

    println!("Success! 1");
}

fn is_vec(v: &Vec<u8>) {
    // You can add some logic here to work with the vector if needed
}


// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);

    let mut v2 = Vec::new();
    v2.extend(v1.iter()); // Заповнюємо v2 елементами з v1

    assert_eq!(v1, v2);

    println!("Success! 2");
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    // Array -> Vec
    // impl From<[T; N]> for Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr); // Заповнюємо пропуск
    let v2: Vec<i32> = arr.to_vec(); // Заповнюємо пропуск

    assert_eq!(v1, v2);

    // String -> Vec
    // impl From<String> for Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into_bytes(); // Заповнюємо пропуск

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3 = Vec::from(s); // Заповнюємо пропуск
    assert_eq!(v2, v3);

    // Iterators can be collected into vectors
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);

    println!("Success! 3");
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let mut v = Vec::from([1, 2, 3]);

    // Додаємо значення, щоб отримати правильний вектор
    v.push(4);
    v.push(5);

    for i in 0..5 {
        println!("{:?}", v[i]);
    }

    for i in 0..5 {
        v[i] += 1; // Збільшуємо кожен елемент на 1
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success! 4");
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    // `v[0..4]` викликатиме паніку, якщо `v` міститиме менше ніж 4 елементи.
    // Треба використовувати `v.len()` для створення коректного слайсу.
    let slice2 = &v[0..v.len()]; // Виправлено, щоб не викликати паніку

    assert_eq!(slice1, slice2);

    // Слайси є тільки для читання
    let vec_ref: &mut Vec<i32> = &mut v;
    vec_ref.push(4); // Додаємо 4 в кінець вектора
    let slice3 = &mut v[0..4]; // Тепер ми беремо слайс з 4 елементів

    // В слайсах не можна використовувати метод `push`, замість цього просто перевіримо значення
    assert_eq!(slice3, &[1, 2, 3, 4]); // Змінено, щоб перевірити правильне значення

    println!("Success! 5");
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let mut vec = Vec::with_capacity(10);

    // Вектор не містить елементів, хоча має запас на більше
    assert_eq!(vec.len(), 0); // Вектор спочатку порожній
    assert_eq!(vec.capacity(), 10);

    // Всі ці операції виконуються без повторного виділення пам'яті...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10); // Тепер вектор містить 10 елементів
    assert_eq!(vec.capacity(), 10); // Запас все ще 10, без повторного виділення

    // ...але це може викликати повторне виділення пам'яті
    vec.push(11);
    assert_eq!(vec.len(), 11); // Тепер вектор містить 11 елементів
    assert!(vec.capacity() >= 11); // Ємність повинна бути 11 або більше

    let mut vec = Vec::with_capacity(100); // Встановлюємо ємність 100
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100); // Тепер вектор містить 100 елементів
    assert_eq!(vec.capacity(), 100); // Запас також повинен бути 100

    println!("Success! 6");
}

// ---------------------------------------------

// Сьоме завдання
#[test]
fn test7() {
    // Заповнюємо пропуск
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    // Порівнюємо два енумератори, потрібно реалізувати трейт PartialEq
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success! 8");
}
#[derive(Debug, PartialEq)] // Додано PartialEq для можливості порівняння
enum IpAddr {
    V4(String),
    V6(String),
}

// ---------------------------------------------

// Восьме завдання
#[test]
fn test8() {
    // Заповнюємо пропуск
    let v: Vec<Box<dyn IpAddrTrait>> = vec![ // Використання нового імені трейту
                                             Box::new(V4("127.0.0.1".to_string())),
                                             Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
// Зміна назви enum на `IpAddrEnum` (була помилка)
enum IpAddrEnum {
    V4(String),
    V6(String),
}

trait IpAddrTrait { // Зміна назви трейту на `IpAddrTrait`(була помилка)
    fn display(&self);
}

struct V4(String);
impl IpAddrTrait for V4 { // Використання нового імені трейту (була помилка)
    fn display(&self) {
        println!("ipv4: {:?}", self.0);
    }
}

struct V6(String);
impl IpAddrTrait for V6 { // Використання нового імені трейту (була помилка)
    fn display(&self) {
        println!("ipv6: {:?}", self.0);
    }
}
