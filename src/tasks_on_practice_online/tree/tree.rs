/* ======================================================
   Написати програму яка:
     малює ялинку в консолі
     ялинка має виглядати як в доданому файлі
     єдиний параметр для конфігурації - кількість трикутників
     в коді бажано використовувати iterators
====================================================== */

#[test]
fn main() {
    let triangles = 5;
    let max_width = 2 * triangles - 1;

    for i in 0..triangles {
        for j in 0..=i {
            let stars = 2 * j + 1;
            let spaces = (max_width - stars) / 2;
            println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
        }
    }

    println!("{}", "*".repeat(max_width));
}
