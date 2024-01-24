pub mod cake {
    pub mod flavor {
        pub const COCONUT: &str = "Coconut 🥥";

        pub mod topping {
            pub const SPRINKLE: &str = "Sprinkle 🧁";
        }
    }
    pub fn is_favorite(name: &str) -> bool {
        name == flavor::COCONUT
    }
}

fn no_warning() {
    #![allow(warnings)]
    let unuse_variable = "No warnings here.";
}

fn warnings() {
    let another_unuse_warning = "we get a warning here!";
}
