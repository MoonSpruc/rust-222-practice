// https://www.hackerrank.com/challenges/climbing-the-leaderboard/problem?isFullScreen=true
/*
Умова:
1. Визначити ранги гравців в аркадній грі, яка використовує систему Dense Ranking
для формування лідерборда.
*/

fn climbingLeaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    // Створюємо унікальний та відсортований лідерборд
    let unique_ranked: Vec<i32> = ranked.iter().cloned().collect::<std::collections::HashSet<_>>().into_iter().collect();
    let mut unique_ranked = unique_ranked;
    unique_ranked.sort_unstable_by(|a, b| b.cmp(a)); // Сортуємо за спаданням

    let mut result = Vec::new();
    let mut index = unique_ranked.len(); // Індекс для проходження по ranked

    // Перебираємо кожен бал гравця
    for score in player {
        // Зменшуємо індекс, поки бал гравця вищий або рівний ранжованому балу
        while index > 0 && score >= &unique_ranked[index - 1] {
            index -= 1;
        }
        // Додаємо ранг (індекс + 1)
        result.push((index + 1) as i32);
    }

    result
}