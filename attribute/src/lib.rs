#![crate_type = "lib"]
#![crate_name = "attribute"]

pub fn public_function() {
    println!("called rary's public_function()");
}

fn private_function() {
    println!("called rary's private_function()");
}

pub fn indirect_access() {
    println!("calld rary's indirect_function()");
    private_function();
}