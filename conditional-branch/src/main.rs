#![allow(unreachable_code)]

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

enum FooBar {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    // if/else
    let n = 5;
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }
    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    };
    println!("{} -> {}", n, big_n);

    // loop
    let mut count = 0u32;
    println!("Let's count until infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }

    // nest & label
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    // while
    let mut n = 1;
    while n < 101 {
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

    // for loop
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustancean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustancean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names);
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustancean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    // match
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);

    // destruct
    let triple = (0, -1, 5);
    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("0, {y}, {z}"),
        (1, ..) => println!("1, .."),
        (.., 2) => println!(".., 2"),
        (3, .., 4) => println!("3, .., 4"),
        _ => println!("It doesn't matter what they are"),
    }
    let array = [1, -2, 6];
    match array {
        [0, second, third] => println!("0, {second}, {third}"),
        [1, _, third] => println!("1, _, {third}"),
        [-1, second, ..] => println!("-1, {second}, .."),
        [3, second, tail @ ..] => println!("3, {}, {:?}", second, tail),
        [first, middle @ .., last] => println!("{}, {:?}. {}", first, middle, last),
    }
    let color = Color::RGB(122, 17, 40);
    match color {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::RGB(r, g, b) => println!("{}, {}, {}", r, g, b),
        Color::HSV(h, s, v) => println!("{}, {}, {}", h, s, v),
        Color::HSL(h, s, l) => println!("{}, {}, {}", h, s, l),
        Color::CMY(c, m, y) => println!("{}, {}, {}", c, m, y),
        Color::CMYK(c, m, y, k) => println!("{}, {}, {}, {}", c, m, y, k),
    }
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    match *reference {
        val => println!("Got a value via dereference: {:?}", val),
    }
    let _not_a_reference = 3;
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. 'mut_value': {:?}", m);
        },
    }
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
        Foo { y: 2, x: i} => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }
    let faa = Foo { x: (1, 2), y: 3 };
    let Foo { x: x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");
    // guard
    let temperature = Temperature::Celsius(35);
    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Celsius", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
    }
    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Fell through"),
    }
    // binding
    match age() {
        0 => println!("0 age"),
        n @ 1 ..= 12 => println!("child: {:?}", n),
        n @ 13 ..= 19 => println!("teen: {:?}", n),
        n => println!("old")
    }
    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interenting... {}", n),
        _ => (),
    }

    // if let
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;
    if let Some(i) = number {
        println!("Matced {:?}", i);
    }
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters!");
    }
    let a = FooBar::Bar;
    let b = FooBar::Baz;
    let c = FooBar::Qux(100);
    if let FooBar::Bar = a {
        println!("a is foobar");
    }
    if let FooBar::Bar = b {
        println!("b is foobar");
    }
    if let FooBar::Qux(i) = c {
        println!("c is foo qux {:?}", i);
    }

    // let-else
    use std::str::FromStr;
    fn get_count_item(s: &str) -> (u64, &str) {
        let mut it = s.split(' ');
        let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
            panic!("Can't segment count item pair: '{s}'");
        };
        let Ok(count) = u64::from_str(count_str) else {
            panic!("Can't parse integer: '{count_str}'");
        };
        (count, item)
    }
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));

    // while let
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9");
            optional = None;
        } else {
            println!("i is {:?}.", i);
            optional = Some(i + 1);
        }
    }
}
