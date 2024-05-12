mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called mu::function()");
}

fn private_function() {
    println!("called my::private_function()");
}

pub fn indirect_access() {
    println!("called my::indirct_access()");
    private_function();
}
