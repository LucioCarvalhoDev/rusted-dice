pub mod dice {
    use rand::prelude::*;
    use regex::Regex;

    pub struct Dice {
        quantity: i32,
        faces: i32,
    }

    impl Dice {
        fn quantity(expression: &str) -> i32 {
            let reg = Regex::new(r"^(\d*)").unwrap();
            match reg.captures(expression) {
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
            let reg: Regex = Regex::new(r"(\d*)$").unwrap();
            match reg.captures(expression) {
                None => 6,
                Some(x) => x.get(0).unwrap().as_str().parse().unwrap(),
            }
        }

        pub fn new(expression: &str) -> Result<Dice, &str> {
            let reg = regex::Regex::new(r"^\d*d\d*$").unwrap();

            if reg.is_match(expression) {
                Result::Ok(Dice {
                    faces: Dice::faces(expression),
                    quantity: Dice::quantity(expression),
                })
            } else {
                Result::Err("Invalid definition")
            }
        }

        pub fn roll(&self) -> Vec<i32> {
            let mut results: Vec<i32> = Vec::new();
            while results.len() < self.quantity as usize {
                results.push(rand::thread_rng().gen_range(1..=self.faces));
            }
            results
        }
    }
}
