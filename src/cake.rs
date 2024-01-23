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


