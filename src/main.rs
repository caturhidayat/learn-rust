mod cake;
// use cake::cake::is_favorite;
fn main() {
    println!("Hello, world!");

    fn double_num(num: u32) -> u32 {
        let double = 2;
        num * double
    }

    println!("hasil perkalian adalah : {}", double_num(10));

    // Mutabily
    // let mut name = "Catur";
    // let mut single_number = 8;

    // name = "Rina Pratiwi";
    // single_number = 10;

    // println!("My name is : {}", name);
    // println!("Display Number : {}", single_number);

    fn question(a: &mut String) {
        a.pop();
        a.push('🔥');
    }

    let mut sentence = String::from("Catur Hidayat Learn Rust.");

    question(&mut sentence);

    print!("{sentence}");

    let mut sentences = String::from("Rina Pratiwi.");
    let immutable_sentence = &mut sentences;

    println!("{}", immutable_sentence);
    println!("{}", sentences);

    struct Cordinate {
        x: i32,
        y: i32,
    }

    let mut cord = Cordinate { x: 80, y: 30 };

    cord.x = 85;
    cord.y = 20;

    println!("Cord x : {}, and y: {}", cord.x, cord.y);

    // Constant Fn
    const fn day_to_second(day: usize) -> usize {
        day * 60 * 60 * 24
    }

    const WEEK_IN_SECOND: usize = day_to_second(7);

    let desember_in_second = day_to_second(31);

    println!("{WEEK_IN_SECOND}");
    println!("{desember_in_second}");

    // Constant Associate
    trait Animal {
        const BIRDIE: &'static str = "Bird 🐤";
    }

    struct Caddy;

    impl Animal for Caddy {}

    println!("{}", Caddy::BIRDIE);

    // Export
    pub(crate) fn print_lemon() {
        println!("lemon 🍋");
    }

    print_lemon();

    let guess = "Coconut";

    println!("{}", cake::cake::flavor::COCONUT);
    println!("{}", cake::cake::flavor::topping::SPRINKLE);
    if cake::cake::is_favorite(guess) {
        println!("That's my favorite cake!");
    } else {
        println!("Not my favorite...");
    }
}
