use add::SubCommand;

mod add;

fn usage_message() -> String {
    return format!(
        "Usage: rlt <subcommand> [options] [args]\n{}",
        "\tadd [options] <filename>|<directory>\n"
    );
}

fn error(message: &str) -> i32 {
    eprintln!("{}", message);
    return 1;
}

fn main() -> Result<(), i32> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(error(&usage_message()));
    }

    let command = &args[1];
    match command.as_str() {
        "add" => {
            let add = add::Add::new(&args.get(2..).expect("no argument"));
            return add.run();
        }
        _ => return Err(error(&usage_message())),
    }
}
