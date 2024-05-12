mod my_mod {
    fn private_fuction() {
        println!("called my_mod::private_fuction()");
    }

    pub fn function() {
        println!("called my_mod::public_function()");
    }

    pub fn indirect_access() {
        println!("called my_mod::indirect_access()");
        private_fuction();
    }

    pub mod nested {
        pub fn function() {
            println!("called my_mod::nested::function()");
        }

        #[allow(dead_code)]
        fn private_fuction() {
            println!("called my_mod::nested::private_fuction()");
        }

        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("called my_mod::nested::public_function_in_my_mod()");
            public_function_in_nested();
        }

        pub(self) fn public_function_in_nested() {
            println!("called my_mod::nested::public_function_in_nested()");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod()");
        }
    }

    pub fn call_public_function_in_my_mod() {
        println!("called my_mod::call_public_function_in_my_mod()");
        nested::public_function_in_my_mod();
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate() {
        println!("called my_mod::public_function_in_crate()");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called my_mod::private_nested::function()");
        }

        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called my_mod::private_nested::restricted_function()");
        }
    }
}

fn function() {
    // private & public
    println!("called function()");
}

mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }

    mod cool {
        pub fn function() {
            println!("called my::cool::function()");
        }
    }

    fn function() {
        println!("called my::function()");
    }

    pub fn indirect_access() {
        println!("called my::indirected_access()");
        self::function();
        function();
        self::cool::function();
        super::function();
        {
            use crate::cool::function as root_fuction;
            root_fuction();
        }
    }
}

use deeply::nested::function as other_function;

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called deeply::nested::function()");
        }
    }
}

mod cool { 
    pub fn function() {
        println!("called cool::function()");
    }
}

fn main() {
    function();
    my_mod::function();
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();
    my_mod::public_function_in_crate();
    // my_mod::nested::public_function_in_my_mod();
    // my_mod::private_fuction();
    // my_mod::nested::private_fuction();
    // my_mod::private_nested::function();
    // my_mod::private_nested::restricted_function();

    // struct
    let open_box = my::OpenBox { contents: "public information" };
    println!("The open box contains: {}", open_box.contents);
    // let closed_box = ClosedBox { contents: "classified information" };
    let _closed_box = my::ClosedBox::new("classified information");
    // println!("The closed box contains: {}", _closed_box.contents);

    // use
    other_function();
    println!("Entering block");
    {
        use crate::deeply::nested::function;
        function();
        println!("Leaving block");
    }
    function();

    // super & self
    my::indirect_access();
}
