use add::SubCommand;

mod add;
mod blob;

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

// make blob
fn main() -> Result<(), i32> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(error(&usage_message()));
    }

    let command = args.get(1).expect("no subcommand");
    match command.as_str() {
        "make-blob" => {
            // generate u8 vector from argv[2]
            blob::Blob::run()?;
            return Ok(());
        }
        "help" => {
            println!("{}", usage_message());
            return Ok(());
        }
        "hash-object" => {
            return Err(error("hash-object is not implemented yet"));
        }
        "add" => {
            let add = add::Add::new(&args.get(2..).expect("no argument"));
            return add.run();
        }
        _ => return Err(error(&usage_message())),
    }
}
