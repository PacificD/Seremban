mod database;

use std::{env, process};

use database::Database;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: sarap [add|rm|ls|help] [args]");
        process::exit(1);
    }

    let command = &args[1];
    let mut db = Database::open(".sarap");

    match command.to_lowercase().as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: sarap add [content]");
                process::exit(1);
            }
            let contents = &args[2..].join(" ");
            db.add_record(&database::Record {
                id: 1,
                content: contents.to_string(),
            })
        }
        "rm" => {
            if args.len() < 3 {
                println!("Usage: sarap rm [id]");
                process::exit(1);
            }
        }
        "ls" => println!("List"),
        _ => println!("Unknown command {command}"),
    };
}
