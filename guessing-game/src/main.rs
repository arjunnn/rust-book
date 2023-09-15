use ferris_says::{self, say};
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io::{stdin, stdout, BufWriter};

fn main() {
    let message = "This is a guessing game";
    let writer = BufWriter::new(stdout().lock());
    let _ = say(message, message.chars().count(), writer);
    let lucky_number: u32 = thread_rng().gen_range(1..=100);
    let mut first_try = true;
    let mut tries = 0;
    loop {
        if first_try {
            println!("Guess a number. Let's see if you can guess today's lucky number: ");
        } else {
            println!("Try again: ");
        }
        let mut guess = "".to_string();
        stdin()
            .read_line(&mut guess)
            .expect("User guess should be read into buffer");
        // println!("Today's lucky number is {lucky_number}");
        let guess = guess.trim();
        println!("This is your guess: {guess}");
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("You should guess a number! {guess} is not a number!");
                continue;
            }
        };
        match guess.cmp(&lucky_number) {
            Ordering::Less => println!("Too small :[ "),
            Ordering::Equal => {
                println!("You guessed it! You win! :D");
                println!("Took you only {tries} tries");
                // say("Congratulations \n Took you only {} tries", 30, writer);
                break;
            }
            Ordering::Greater => println!("Too big :o"),
        }
        first_try = false;
        tries += 1;
        println!("===========");
    }
}
