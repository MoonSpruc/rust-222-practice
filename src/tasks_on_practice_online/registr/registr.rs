/* ======================================================
   Написати функцію яка:
     міняє регістр з верхнього на нижній
     міняє регістр з нижнього на верхнього
====================================================== */

fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().next().unwrap()
            } else {
                c.to_uppercase().next().unwrap()
            }
        })
        .collect()
}

#[test]
fn test() {
    let input = "Hello".to_string();
    let inverted = invert_the_case(input.clone());
    println!("Оригінальне: {}, Перероблене: {}", input, inverted);

    let input2 = "Привіт".to_string();
    let inverted2 = invert_the_case(input2.clone());
    println!("Оригінальне: {}, Перероблене: {}", input2, inverted2);
}