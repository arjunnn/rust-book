use ferris_says::{self, say};
use std::io::{stdin, stdout, BufWriter};

fn main() {
    // Program to convert Celcius <-> Farenheit.
    let writer = BufWriter::new(stdout().lock());
    match say("Temperature Converter C <-> F <-> K", 100, writer) {
        Ok(_) => {}
        Err(message) => println!("Error occured while writing to buffer: {message}"),
    };
    get_input();
}

fn get_input() {
    println!("Enter temperature value:");
    let value = get_value();
    println!("Enter temperature unit:");
    let unit = get_unit();
    if unit == 'C' {
        let new_value = c_to_f(value);
        println!("{value}C = {new_value:.2}F = {:.2}K", value + 273.15);
    } else if unit == 'F' {
        let new_value = f_to_c(value);
        println!("{value}F = {new_value:.2}C = {:.2}K", new_value + 273.15);
    } else if unit == 'K' {
        let c = value - 273.15;
        println!("{value}K = {c:.2}C = {:.2}", c_to_f(c));
    }
}

fn c_to_f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn get_value() -> f64 {
    'input_value_loop: loop {
        let mut value: String = "".to_string();
        stdin()
            .read_line(&mut value)
            .expect("Error while reading input");
        match value.trim().parse() {
            Ok(result) => break 'input_value_loop result,
            Err(_) => {
                println!("Input value should be a number! Try again:");
                continue;
            }
        }
    }
}
fn get_unit() -> char {
    const ERROR_MESSAGE: &str = "Unit should be one of C, F & K! Try again:";
    'input_unit_loop: loop {
        let mut unit: String = "".to_string();
        stdin()
            .read_line(&mut unit)
            .expect("Error while reading input");
        match unit.trim().to_uppercase().parse() {
            Ok(result) => {
                if result == 'C' || result == 'F' || result == 'K' {
                    break 'input_unit_loop result;
                } else {
                    println!("{}", ERROR_MESSAGE.to_owned());
                    continue;
                }
            }
            Err(_) => {
                println!("{}", ERROR_MESSAGE.to_owned());
                continue;
            }
        }
    }
}
