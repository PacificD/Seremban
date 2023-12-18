mod database;

use std::env;

use database::Database;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: sarap [add|rm|ls|help] [args]");
        return;
    }

    let command = &args[1];
    let mut db = Database::open(".sarap");

    match command.to_lowercase().as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: sarap add [content]");
                return;
            }
            let contents = &args[2..].join(" ");
            let id = db
                .read_records()
                .last()
                .map(|item| item.id + 1)
                .unwrap_or(1);
            db.add_record(&database::Record {
                id,
                content: contents.to_string(),
            })
        }
        "rm" => {
            if args.len() < 3 {
                println!("Usage: sarap rm [id]");
                return;
            }
        }
        "ls" => {
            let records = db.read_records();
            if records.is_empty() {
                println!("No records. You can add one with 'sarap add [content]");
            }
            for record in records {
                println!("📄 {}: {}", record.id, record.content);
            }
        }
        _ => println!("Unknown command {command}"),
    };
}
