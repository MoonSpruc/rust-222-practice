// https://www.hackerrank.com/challenges/migratory-birds/problem?isFullScreen=true
/*
Умова:
1. Визначення найчастіше зустріченого типу птахів
*/

use std::collections::HashMap;
fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut freq_map: HashMap<i32, i32> = HashMap::new();

    // Підраховуємо кількість появ кожного типу птаха
    for &bird in arr {
        *freq_map.entry(bird).or_insert(0) += 1;
    }

    // Знаходимо тип птаха, який зустрічається найчастіше
    let mut most_frequent = 0;
    let mut most_frequent_id = i32::MAX;

    for (&bird_id, &count) in &freq_map {
        if count > most_frequent || (count == most_frequent && bird_id < most_frequent_id) {
            most_frequent = count;
            most_frequent_id = bird_id;
        }
    }

    most_frequent_id
}