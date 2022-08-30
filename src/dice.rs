pub mod dice {
    use lazy_static::lazy_static;
    use rand::prelude::*;
    use regex::Regex;

    pub struct RolledOut {
        pub values: Vec<i32>,
        pub definition: String,
    }

    impl RolledOut {
        pub fn sum(&self) -> i32 {
            let mut res = 0;

            for i in 1..=self.values.len() {
                res += self.values[i - 1];
            }

            res
        }

        pub fn max(&self) -> i32 {
            let mut biggest = 0;

            for i in 1..=self.values.len() {
                let cur = self.values[i - 1];
                if cur > biggest {
                    biggest = cur;
                }
            }

            biggest
        }

        pub fn min(&self) -> i32 {
            let mut lesser = i32::MAX;

            for i in 1..=self.values.len() {
                let cur = self.values[i - 1];
                if cur < lesser {
                    lesser = cur;
                }
            }

            lesser
        }
    }

    pub struct Dice {
        pub quantity: i32,
        pub faces: i32,
        pub definition: String,
    }

    impl Dice {
        fn quantity(expression: &str) -> i32 {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^(\d*)").unwrap();
            }
            match RE.captures(expression) {
                None => {
                    println!("IMPLICIT QUANTITY");
                    1
                }
                Some(x) => {
                    let capture = x.get(0).unwrap().as_str();

                    if capture == "" {
                        return 1;
                    }

                    capture.parse().unwrap()
                }
            }
        }

        fn faces(expression: &str) -> i32 {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"(\d*)$").unwrap();
            }
            match RE.captures(expression) {
                None => 6,
                Some(x) => x.get(0).unwrap().as_str().parse().unwrap(),
            }
        }

        pub fn new(expression: &str) -> Result<Dice, String> {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^\d*d\d*$").unwrap();
            }

            if RE.is_match(expression) {
                Result::Ok(Dice {
                    faces: Dice::faces(expression),
                    quantity: Dice::quantity(expression),
                    definition: expression.to_owned(),
                })
            } else {
                let error_msg = format!("dice invalid constructor: {}", 1);
                Result::Err(error_msg)
            }
        }

        pub fn roll(&self) -> RolledOut {
            let mut results: Vec<i32> = Vec::new();
            while results.len() < self.quantity as usize {
                results.push(rand::thread_rng().gen_range(1..=self.faces));
            }
            RolledOut {
                values: results,
                definition: self.definition.to_owned(),
            }
        }
    }
}
