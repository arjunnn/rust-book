use ferris_says::{self, say};
use std::io::{stdout, BufWriter};
fn main() {
    let writer = BufWriter::new(stdout().lock());
    say("🎄 Twelve days of Christmas 🎶", 100, writer)
        .expect("Greeting should be printed to stdout.");
    const GIFTS: [&str; 12] = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "gold rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    const NUMBERS: [&str; 12] = [
        "a", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven",
        "Twelve",
    ];

    // days loop
    for day in 1..=12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            DAYS[day - 1]
        );
        let gift = GIFTS[day - 1];
        let mut number = NUMBERS[day - 1];
        let number_as_string: String;
        if day == 1 {
            number_as_string = number.to_uppercase();
            number = number_as_string.as_str();
        }
        println!("{} {}", number, gift);
        for previous_day in (1..day).rev() {
            let number = NUMBERS[previous_day - 1];
            if day != 1 && previous_day == 1 {
                print!("And ")
            }
            print!("{} {}", number, GIFTS[previous_day - 1]);
            if previous_day == 1 {
                println!();
            } else {
                println!(",");
            }
        }
        println!();
    }
}
