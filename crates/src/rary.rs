pub fn public_function() {
    println!("called rary's public_function()");
}

fn private_fuction() {
    println!("called rary's private_function()");
}

pub fn indirect_access() {
    println!("called rary's indirect_access()");
    private_fuction();
}
