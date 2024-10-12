// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem?isFullScreen=true
/*
Умова:
1. Написати програму, яка буде рахувати кількість разів, коли Марія
побила свої рекорди за максимальним і мінімальним кількістю очок у грі
*/

fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let mut min_record = scores[0];  // початковий рекорд мінімуму
    let mut max_record = scores[0];  // початковий рекорд максимуму
    let mut min_count = 0;  // кількість побиттів мінімуму
    let mut max_count = 0;  // кількість побиттів максимуму

    for &score in scores.iter().skip(1) {
        if score > max_record {
            max_record = score;
            max_count += 1;
        }
        if score < min_record {
            min_record = score;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
}