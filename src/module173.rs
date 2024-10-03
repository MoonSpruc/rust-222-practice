// Перше завдання
struct DoubleRef<'a, 'b, T>
where
    'a: 'b, // Вказуємо, що термін життя 'a є меншим або рівним 'b
{
    r: &'a T,
    s: &'b T,
}
#[test]
fn test1() {
    println!("Success! 1");
}

// ---------------------------------------------

// Друге завдання
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
    where
        'a: 'b, // Вказуємо, що термін життя 'a є більшим або рівним 'b
    {
        println!("Attention please: {}", announcement);
        announcement // Повертаємо посилання на `announcement`
    }
}
#[test]
fn test2() {
    println!("Success! 2");
}

// ---------------------------------------------

// Третє завдання
fn f<'a, 'b>(x: &'a i32, y: &'b mut i32) {
    *y = *x;  // Присвоюємо значення, яке 'x' вказує, в 'y'
    let r: &'a i32 = &0; // Використовуємо термін життя 'a' для 'r'
}
#[test]
fn test3() {
    let a = 42;
    let mut b = 0;
    f(&a, &mut b);
    println!("Success! 3 b = {}", b);
}

// ---------------------------------------------

// Четверте завдання
fn call_on_ref_zero<F>(f: F)
where
    F: for<'a> Fn(&'a i32), // Додаємо HRTB, щоб F могло приймати посилання на i32 з будь-яким терміном життя
{
    let zero = 0;
    f(&zero); // Викликаємо функцію з посиланням на нуль
}
#[test]
fn test4() {
    call_on_ref_zero(|x| {
        println!("Value: {}", x);
    });
    println!("Success! 4");
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let mut data = 10;
    {
        let ref1 = &mut data; // Створюємо змінне посилання ref1
        *ref1 += 1;          // Змінюємо data через ref1
    } // ref1 виходить з області видимості тут

    {
        let ref2 = &mut data; // Тепер можемо створити ref2
        *ref2 += 2;           // Змінюємо data через ref2
    } // ref2 виходить з області видимості тут

    println!("{}", data); // Виводимо значення data
}

// ---------------------------------------------

// Шосте завдання
// struct Interface<'a> {
//     manager: &'a mut Manager<'a>,
// }
//
// impl<'a> Interface<'a> {
//     pub fn noop(&mut self) {
//         println!("interface consumed");
//     }
// }
//
// struct Manager<'a> {
//     text: &'a str,
// }
//
// struct List<'a> {
//     manager: Manager<'a>,
// }
//
// impl<'a> List<'a> {
//     pub fn get_interface(&mut self) -> Interface<'_> {
//         Interface {
//             manager: &mut self.manager,
//         }
//     }
// }
// #[test]
// fn test6() {
//     let mut list = List {
//         manager: Manager {
//             text: "hello",
//         },
//     };
//
//     {
//         let mut interface = list.get_interface(); // Викликаємо get_interface
//         interface.noop(); // Викликаємо noop на interface
//     } // Виходимо з блоку, щоб звільнити borrow
//
//     println!("Interface should be dropped here and the borrow released");
//
//     use_list(&list); // Викликаємо use_list
// }
// fn use_list(list: &List) {
//     println!("{}", list.manager.text);
// }
