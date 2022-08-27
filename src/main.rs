mod dice;
use crate::dice::dice::Dice;
use std::io::{self, Write};

fn main() {
    print!("roll: ");
    match io::stdout().flush() {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    };

    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line");

    let dice = Dice::new(expression.trim()).unwrap();

    println!("result: {:?}", dice.roll());
    // println!("found: {}", dice.quantity);
    let mut end_program = String::new();
    io::stdin().read_line(&mut end_program).unwrap();
}
