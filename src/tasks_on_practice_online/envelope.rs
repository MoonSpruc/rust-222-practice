#[test]
fn test() {
    const W: u32 = 45; // ширина
    const H: u32 = 20; // висота

    for y in 0..H {
        for x in 0..W {
            // Горизонтальні лінії (верхня та нижня межі)
            let is_horizontal = y == 0 || y == H - 1;
            // Вертикальні лінії (ліва та права межі)
            let is_vertical = x == 0 || x == W - 1;
            // Діагональ зліва направо
            let is_diagonal1 = x == (y * (W - 1)) / (H - 1);
            // Діагональ справа наліво
            let is_diagonal2 = x == (H - 1 - y) * (W - 1) / (H - 1);

            let c = if is_horizontal || is_vertical || is_diagonal1 || is_diagonal2 {
                '*'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}
