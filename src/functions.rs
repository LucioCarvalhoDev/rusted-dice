pub mod functions {
    use crate::dice::dice::{Dice, RolledOut};

    enum FuncType {
        Sum,
        Max,
        Min,
    }
    pub struct Func {
        mode: FuncType,
        pub dice: String,
        pub params: RolledOut,
        pub definition: String,
    }

    impl Func {
        pub fn run(&self) -> i32 {
            match self.mode {
                FuncType::Sum => self.params.sum(),
                FuncType::Max => self.params.max(),
                FuncType::Min => self.params.min(),
            }
        }

        pub fn new(definition: &str) -> Func {
            let reg_mode = regex::Regex::new(r"(\w{3})").unwrap();
            let reg_dice = regex::Regex::new(r"(\d+d\d+)").unwrap();

            let dice_definition = match reg_dice.captures(definition).unwrap().get(0) {
                Some(cap) => cap.as_str(),
                None => "sum",
            };

            let dice_result = Dice::new(dice_definition).unwrap().roll();

            //let func_type =
            let mode_str = reg_mode
                .captures(definition)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str();

            let func_type = {
                if mode_str == "sum" {
                    FuncType::Sum
                } else if mode_str == "max" {
                    FuncType::Max
                } else if mode_str == "min" {
                    FuncType::Min
                } else {
                    FuncType::Max
                }
            };

            return Func {
                mode: func_type,
                params: dice_result,
                dice: dice_definition.to_owned(),
                definition: definition.to_owned(),
            };
        }
    }
}
