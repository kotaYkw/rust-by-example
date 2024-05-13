fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* runnig linux!");
}

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    used_function();

    // cfg
    are_you_on_linux();
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitly linux!");
    } else {
        println!("Yes. It's definitly *not* linux!");
    }

    conditional_function();
}
