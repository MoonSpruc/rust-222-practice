// https://www.hackerrank.com/challenges/counting-valleys/problem?isFullScreen=true
/*
Умова:
1. Обробити кроки туриста і визначити, скільки разів він входив у долину та виходив з неї,
починаючи і закінчуючи на рівні моря.
*/

fn countingValleys(steps: i32, path: &str) -> i32 {
    let mut altitude = 0; // Поточна висота
    let mut valley_count = 0; // Лічильник долин
    let mut in_valley = false; // Чи знаходимося в долині

    for step in path.chars() {
        // Змінюємо висоту в залежності від кроку
        if step == 'U' {
            altitude += 1; // Підйом
        } else if step == 'D' {
            altitude -= 1; // Спуск
        }

        // Перевіряємо, чи увійшли в долину
        if altitude < 0 && !in_valley {
            in_valley = true; // Входимо в долину
        }

        // Перевіряємо, чи вийшли з долини
        if altitude == 0 && in_valley {
            valley_count += 1; // Збільшуємо лічильник долин
            in_valley = false; // Виходимо з долини
        }
    }

    valley_count // Повертаємо кількість долин
}