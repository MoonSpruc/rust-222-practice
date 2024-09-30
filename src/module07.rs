// Перше завдання
#[test]
fn test1() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    let n = 5;

    let big_n = if n < 10 && n > -10 {
        println!("{} is a small number, increase ten-fold", n);
        10 * n
    } else {
        println!("{} is a big number, halve the number", n);
        n / 2
    };

    println!("{} -> {}", n, big_n);
}

// ---------------------------------------------

// Третє завдання
#[test]
fn test3() {
    for n in 1..100 { //діапазон не включає 100
        if n == 100 {
            panic!("NEVER LET THIS RUN");
        }
    }

    println!("Success! 3");
}

// ---------------------------------------------

// Четверте завдання
#[test]
fn test4() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names {
        // Do something with name...
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with n...
    }

    println!("{:?}", numbers);
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let a = [4, 3, 2, 1];

    // Iterate the indexing and value in 'a'
    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    // A counter variable
    let mut n = 1;

    // Loop while the condition is true
    while n <= 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    println!("n reached {}, so loop is over", n);
}

// ---------------------------------------------

// Сьоме завдання
#[test]
fn test7() {
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);

    println!("Success! 7");
}

// ---------------------------------------------

// Восьме завдання
#[test]
fn test8() {
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n += 1;
            continue;
        }

        break; // Завершуємо цикл, коли n дорівнює 66
    }

    assert_eq!(n, 66);

    println!("Success! 8");
}

// ---------------------------------------------

// Дев'яте завдання
#[test]
fn test9() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }

    assert_eq!(count, 5);

    println!("Success! 9");
}

// ---------------------------------------------

// Десяте завдання
#[test]
fn test10() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break 20;
        }
    };

    assert_eq!(result, 20);

    println!("Success! 10");
}

// ---------------------------------------------

// Одинадцяте завдання
#[test]
fn test11() {
    let mut count = 0;
    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1; // вихід із першого внутрішнього циклу, коли count >= 20
            }
            count += 2;
        }

        count += 5; // Тут count = 20 + 5 = 25

        'inner2: loop {
            if count >= 30 {
                break 'outer; // вихід із зовнішнього циклу, коли count >= 30
            }
            count += 5; // збільшуємо count ще на 5, щоб дійти до 30
        }
    }

    assert!(count == 30);

    println!("Success! 11");
}