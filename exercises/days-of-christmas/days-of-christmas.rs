const GIFTS: [&str; 12] = [
    "Twelve drummers drumming;",
    "Eleven pipers piping;",
    "Ten lords a-leaping;",
    "Nine ladies dancing;",
    "Eight maids a-milking;",
    "Seven swans a-swimming;",
    "Six geese a-laying;",
    "Five gold rings;",
    "Four calling birds;",
    "Three French hens;",
    "Two turtle doves;",
    "partridge in a pear tree.",  // needs article
];

const ORDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth",
    "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
];

fn main() {
    println!("Welcome to MY_PARTRIDGE");
    println!("A blazingly fast Christmas carol, written in Rust");
    println!("v1.0");

    for day in 1..=12 {
        println!();
        print_day(day);
    }
}

fn print_day(day: usize) {
    println!("On the {} day of Christmas, my true love gave to me:", ORDINALS[day - 1]);
    for (i, gift) in GIFTS.iter().enumerate().skip(12-day) {
        if i == 11 {
            print!("{}", if day == 1 {"A "} else {"And a "});
        }
        println!("{}", gift);
    }
}
