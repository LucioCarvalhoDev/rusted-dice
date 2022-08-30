mod dice;
mod functions;
use crate::functions::functions::Func;
use eval::eval;
use regex::Regex;
// use std::io::{self, Write};
use lazy_static::lazy_static;

lazy_static! {
    static ref REG_FUNC: Regex = regex::Regex::new(r"(\w{3}\(\d+d\d+\))").unwrap();
}

fn main() {
    fn parser(expression: &str) {
        // let reg_dice = regex::Regex::new(r"(\d+d\d+)").unwrap();

        println!("\n{}\n", expression);

        let mut edited_expression = expression.to_owned();

        for cap in REG_FUNC.captures_iter(expression) {
            let func_definition = cap.get(0).unwrap().as_str();
            let func1 = Func::new(func_definition);
            let result = func1.run().to_string();
            edited_expression = str::replace(&edited_expression[..], func_definition, &result);
            println!(
                "{} => {:?} => {}",
                func_definition, func1.params.values, result
            );
        }

        println!(
            "\n{} => {}",
            edited_expression,
            eval(&edited_expression[..]).unwrap()
        );
    }

    parser("sum(2d6) - sum(1d20) - max(5d4) + 1");
}

#[cfg(test)]
mod tests {
    use crate::dice::dice::Dice;
    use std::io::{self, Write};

    #[test]
    fn dice_performance() {
        use std::time::Instant;

        let times = 1000;
        let dice_def = "10d6";

        let now = Instant::now();
        for _i in 1..=times {
            let dice = Dice::new(dice_def).unwrap();
            dice.roll();
        }
        let elapsed = now.elapsed();
        println!("{} 10d6 rolled out in {:.2?}", times, elapsed);
    }

    #[test]
    fn roll_dice() {
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
        let roll_result = dice.roll();

        println!("result: {:?}", roll_result.values);
    }
}
