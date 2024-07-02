#![allow(dead_code)]

use std::{num::ParseIntError, result};

#[cfg(panic = "unwind")]
fn ah() {
    println!("Split it out!!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party. Run!!");
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        // panic!("AAaaaa!!");
        ah();
    } else {
        println!("Some refreshing {} is all I need", beverage);
    }
}

fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well."),
    }
}

fn drink_unwrap(drink: Option<&str>) {
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("AAAaaaa");
    }
    println!("I love {}s!!!", inside);
}

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Patato,
}

#[derive(Debug)]
struct Peeled(Food);

#[derive(Debug)]
struct Chopped(Food);

#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm, I love {:?}", food),
        None => println!("Oh no It wasn't editable."),
    }
}

#[derive(Debug)]
enum Feast {
    CordonBleu,
    Steak,
    Sushi,
}

#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

fn have_ingredients(feast: Feast) -> Option<Feast> {
    match feast {
        Feast::Sushi => None,
        _ => Some(feast),
    }
}

fn have_recipe(feast: Feast) -> Option<Feast> {
    match feast {
        Feast::CordonBleu => None,
        _ => Some(feast),
    }
}

fn cookable_v1(feast: Feast) -> Option<Feast> {
    match have_recipe(feast) {
        None => None,
        Some(feast) => have_ingredients(feast),
    }
}

fn cookable_v3(feast: Feast) -> Option<Feast> {
    have_recipe(feast).and_then(have_ingredients)
}

fn cookable_v2(feast: Feast) -> Option<Feast> {
    have_recipe(feast).map(have_ingredients).flatten()
}

fn eat_feast(feast: Feast, day: Day) {
    match cookable_v3(feast) {
        Some(feast) => println!("Yay! On {:?} we get to eat {:?}", day, feast),
        None => println!("Oh no. We get to to eat on {:?}?", day),
    }
}

#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() -> Result<(), ParseIntError> {
    drink("water");
    drink("lemonade");

    // Option & unwrap
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;
    give_adult(water);
    give_adult(lemonade);
    give_adult(void);
    let coffee = Some("coffee");
    let _nothig: Option<&str> = None;
    drink_unwrap(coffee);
    // drink_unwrap(nothig);

    // unpack
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 43922222,
            }),
        }),
    };
    assert_eq!(p.work_phone_area_code(), Some(61));

    // Combinators:map
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;
    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(potato);
    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);

    // Combinators:and_then
    let (cordon_bleu, steak, sushi) = (Feast::CordonBleu, Feast::Steak, Feast::Sushi);
    eat_feast(cordon_bleu, Day::Monday);
    eat_feast(steak, Day::Tuesday);
    eat_feast(sushi, Day::Wednesday);

    // unpacking options
    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = || {
        println!("Providing kemon as fallback");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);
    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
    println!("should_be_apple is: {:?}", should_be_apple);
    println!("my_apple is unchanged: {:?}", my_apple);

    let twenty = multiply("10", "2");
    println!("double is {}", twenty);
    // let tt = multiply("t", "2");
    // println!("double is P {}", tt);

    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
