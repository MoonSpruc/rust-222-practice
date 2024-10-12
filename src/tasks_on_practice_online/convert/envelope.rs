// #[test]
// fn test() {
//     const W: u32 = 25; // ширина
//     const H: u32 = 10; // висота
//
//     for y in 0..H {
//         for x in 0..W {
//             // Горизонтальні лінії (верхня та нижня межі)
//             let is_horizontal = y == 0 || y == H - 1;
//             // Вертикальні лінії (ліва та права межі)
//             let is_vertical = x == 0 || x == W - 1;
//             // Діагональ зліва направо
//             let is_diagonal1 = x == (y * (W - 1)) / (H - 1);
//             // Діагональ справа наліво
//             let is_diagonal2 = x == (H - 1 - y) * (W - 1) / (H - 1);
//
//             let c = if is_horizontal || is_vertical || is_diagonal1 || is_diagonal2 {
//                 '*'
//             } else {
//                 ' '
//             };
//             print!("{}", c);
//         }
//         println!();
//     // Зробити так щоб діагоналі виглядали більш красиво
//     }
// }

#[test]
fn test2() {
    const W: u32 = 25;
    const H: u32 = 10;
    let k = W as f32 / H as f32; // 2.5

    fn mul(a: u32, b: f32) -> u32 {
        (a as f32 * b) as u32
    }

    for y in 1..=H {
        for x in 1..=W {
            let c: char = match (x, y) {
                (_, 1 | H) => '-',
                (1 | W, _) => '|',
                _ if x == mul(y, k) => '\\',
                _ if x == W - mul(y, k) => '/',
                _ => ' ',
            };
            print!("{}", c);
        }
        println!();
    }
}
