struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }
}
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function!");
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    // function
    fizzbuzz_to(100);

    // associated functions & method
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("perimeter: {}", rectangle.perimeter());
    println!("area: {}", rectangle.area());
    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    square.translate(1.0, 1.0);
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // closure
    let outer_val = 42;
    let closure_annotated = |i: i32| -> i32 { i + outer_val };
    let closure_inferred = |i| i + outer_val;
    println!("closure_annnotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    let one = || 1;
    println!("closure returning one: {}", one());
    use std::mem;
    let color = String::from("green");
    let print = || println!("color: {}", color);
    print();
    let _reborrow = &color;
    print();
    let _color_moved = color;
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };
    inc();
    inc();
    let _count_reborrow = &mut count;
    let moveble = Box::new(3);
    let consume = || {
        println!("movable: {:?}", moveble);
        mem::drop(moveble);
    };
    consume();
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    let diary = || {
        println!("I said {}", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("Now I can sleep. zzzzz");
        mem::drop(farewell);
    };
    apply(diary);
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
    let x = 7;
    let print = || println!("{}", x);
    apply(print);
    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(function);
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    fn_plain();
    fn_mut();
    fn_once();
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.iter().any(|&x| x == 2));
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.iter().any(|&x| x == 2));
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Fins 2 in vec2: {:?}", into_iter.find(|&x| x == 2));
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!("Fins 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));
    let vec = vec![1, 9, 3, 3, 13, 2];
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));
    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);

    // Higher Order Functions
    let upper = 1000;
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);
    let sum_of_squared_odd_numbers: u32 = 
        (0..).map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .sum();
    println!("functional style: {}", sum_of_squared_odd_numbers);

    // divergence function
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i%2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
