#[derive(Debug)]
pub struct Add {
    options: Vec<String>,
    argument: Vec<String>,
}

pub trait SubCommand {
    fn usage_message(&self) -> String;

    fn new(args: &[String]) -> Self;

    fn run(&self) -> Result<(), i32>;
}

impl SubCommand for Add {
    fn usage_message(&self) -> String {
        return format!("Usage: rlt add [-A|--all] [--chmod=(+|-)x] [-u|--update] <path-spec>\n");
    }

    fn new(args: &[String]) -> Self {
        let mut options = Vec::new();
        let mut argument = Vec::new();
        for arg in args {
            if arg.starts_with("-") {
                options.push(arg.to_string());
            } else {
                argument.push(arg.to_string());
            }
        }
        return Add { options, argument };
    }

    fn run(&self) -> Result<(), i32> {
        println!("add {:?}", self.argument);
        if self.argument.len() == 0 {
            return Err(1);
        }
        return Ok(());
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
