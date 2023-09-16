// trait AddTrait {
//     fn usage(&self) -> String;
//     fn new(args: &[String]) -> Self;
//     fn run(&self);
// }

#[derive(Debug)]
pub struct Add {
    options: Vec<String>,
    argument: Vec<String>,
}

impl Add {
    pub fn usage(&self) -> String {
        format!("add {:?}", self.argument)
    }

    pub fn new(args: &[String]) -> Self {
        let mut options = Vec::new();
        let mut argument = Vec::new();
        for arg in args {
            if arg.starts_with("-") {
                options.push(arg.to_string());
            } else {
                argument.push(arg.to_string());
            }
        }
        Add { options, argument }
    }

    pub fn run(&self) {
        println!("add {:?}", self.argument);
    }
}

// add test

#[cfg(test)]
mod test {
    #[test]
    fn test_usage() {
        assert!(true)
    }
}
