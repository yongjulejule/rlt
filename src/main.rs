mod add;
use std::collections::HashMap;

fn usage() {
    eprintln!(
        "Usage: {}\n",
        std::env::args().nth(0).unwrap().split('/').last().unwrap()
    );
    eprintln!("\tadd <filename>|<directory>");
    // eprintln!("\tcat-file <type> <object>");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        usage();
        std::process::exit(1);
    }
    // args.iter().for_each(|arg| println!("{}", arg));

    let command = &args[1];
    match command.as_str() {
        "add" => {
            let add = add::Add::new(&args.get(2..).expect("no argument"));
            add.run();
        }
        _ => usage(),
    }

    let teams = vec![
        String::from("Blue"),
        String::from("Yellow"),
        String::from("Yellow"),
        String::from("Yellow"),
        String::from("Yellow"),
        "asd".to_string(),
        "dsa".to_string(),
    ];
    let initial_scores = vec![10, 50, 42, 42, 42, 42, 42, 42];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
    ()
}
