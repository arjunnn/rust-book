use ferris_says::{self, say};
use std::io::{stdin, stdout, BufWriter};
fn main() {
    println!("Hello, world!");
    let writer = BufWriter::new(stdout().lock());
    say("Nth Fibonacci Seqeunce Number", 100, writer).expect("Message should be printed to stdout");
    println!("The Fibonacci sequence starts with 0, 1, 1, 2, 3, 5... and continues in the orer where each number in the sequence is the sum of the previous two numbers in the sequence.");
    let mut sequence: [u128; 3] = [0, 1, 1];
    println!("Enter value  of N:");
    let mut n = "".to_string();
    stdin().read_line(&mut n).expect("Failed to read input");
    let n: u128 = n.trim().parse().expect("Input should be a number");
    if n == 1 {
        println!("{}", sequence[0]);
    } else if n == 2 {
        println!("{}", sequence[1]);
    } else if n == 3 {
        println!("{}", sequence[2]);
    } else {
        for _ in 4..=n {
            sequence[0] = sequence[1];
            sequence[1] = sequence[2];
            sequence[2] = sequence[0] + sequence[1];
        }
        println!("{}", sequence[2]);
    }
}
