// Перше завдання
#[test]
fn test1() {
    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(42)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('b'));

    println!("Success! 1");
}
struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

// ---------------------------------------------

// Друге завдання
#[test]
fn test2() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success! 2");
}
// Implement the generic function below.
fn sum<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    a + b
}

// ---------------------------------------------

// Третє завдання
#[derive(Debug)]
struct Point3<T> {
    x: T,
    y: T,
}
#[test]
fn test3() {
    let integer = Point3 { x: 5, y: 10 };
    let float = Point3 { x: 1.0, y: 4.0 };

    println!("Success! 3");
}

// ---------------------------------------------

// Четверте завдання
struct Point4<X, Y> {
    x: X,
    y: Y,
}
#[test]
fn test4() {
    // DON'T modify this code.
    let p = Point4 { x: 5, y: "hello".to_string() };

    println!("Success! 4");
}

// ---------------------------------------------

// П'яте завдання
#[test]
fn test5() {
    let x = Val { val: 5.0 };
    let y = Val { val: "hello".to_string() };
    println!("{}, {}", x.value(), y.value());
}
struct Val<T> {
    val: T,
}

impl Val<f64> {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl Val<String> {
    fn value(&self) -> &String {
        &self.val
    }
}

// ---------------------------------------------

// Шосте завдання
#[test]
fn test6() {
    let p1 = Point6 { x: 5, y: 10 };
    let p2 = Point6 { x: "Hello", y: '中' };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success! 6");
}
struct Point6<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point6<T, U> {
    // Implement mixup to make it work, DON'T modify other code.
    fn mixup<V, W>(self, other: Point6<V, W>) -> Point6<T, W> {
        Point6 {
            x: self.x,
            y: other.y,
        }
    }
}

// ---------------------------------------------

// Сьоме завдання
#[test]
fn test7() {
    // Use f32 for Point
    let p = Point7 { x: 5.0f32, y: 10.0f32 }; // Use f32 instead of i32
    println!("{}", p.distance_from_origin());
}
struct Point7<T> {
    x: T,
    y: T,
}

impl Point7<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
