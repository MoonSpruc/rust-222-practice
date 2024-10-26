/* ======================================================
   Написати функцію яка:
    рахує мінімальну кількість переносу грузу щоб на всіх кораблях був однаковий груз
    fn count_permutation(shipments: &Vec<u32>) -> usize

    генерації Vec<32> які можуть бути розподілені однаково.
    fn gen_shipments(n: usize) -> Vec<u32>

   Чи завжди можливо всі кораблі забезпечити однаковою кількість грузу?

   Забезпечити всі кораблі однаковою кількістю вантажу можливо не завжди.
   Для цього потрібно, щоб сума вантажу на всіх кораблях ділилася на кількість кораблів без залишку.
   Якщо ця умова не виконується, то рівномірно розподілити вантаж неможливо.

   Як буде виглядати сігнатура в іншому випадку?

   fn count_permutation(shipments: &Vec<u32>) -> Result<usize, &'static str> "можна спробувати"
====================================================== */


// Задача 1
fn count_permutation(shipments: &Vec<u32>) -> usize {
    let n = shipments.len() as u32;
    let total_weight: u32 = shipments.iter().sum();

    if total_weight % n != 0 {
        return usize::MAX;
    }

    let target_weight = total_weight / n;
    let mut moves: usize = 0;

    for &weight in shipments {
        if weight > target_weight {
            moves += (weight - target_weight) as usize;
        }
    }

    moves
}

#[test]
fn main() {
    let shipments = vec![1, 1, 1, 1, 6];
    let result = count_permutation(&shipments);
    println!("{}", result); // Виведе 4
}




use rand::Rng;

// Задача 2
// fn gen_shipments(n: usize) -> Vec<u32> {
//     let mut rng = rand::thread_rng();
//     let mut shipments: Vec<u32> = (0..n).map(|_| rng.gen_range(1..=10)).collect();
//
//     let total_weight: u32 = shipments.iter().sum();
//     let remainder = total_weight % n as u32;
//     if remainder != 0 {
//         shipments[0] += (n as u32 - remainder);
//     }
//
//     shipments
// }
//
// #[test]
// fn main() {
//     let n = 5;
//     let shipments = gen_shipments(n);
//     println!("{:?}", shipments);
// }
