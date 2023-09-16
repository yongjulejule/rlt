fn usage() {
    eprintln!(
        "Usage: {}\n",
        std::env::args().nth(0).unwrap().split('/').last().unwrap()
    );
    eprintln!("\tadd <filename>|<directory>");
    eprintln!("\tcat-file <type> <object>");
    eprintln!("\tcheck-ignore <pattern>...");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        usage();
    }

    let command = args[1].clone();
    match command.as_str() {
        "add" => Add {
            filename: args[2].clone(),
        }
        .run(),
        "cat-file" => println!("cat-file"),
        _ => println!("default"),
    }

    args.iter().for_each(|arg| println!("{}", arg));

    println!("Hello, world!");
}
