use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct Record {
    pub id: u32,
    pub content: String,
}

pub struct Database {
    pub file: File,
}

impl Database {
    pub fn open(file_name: &str) -> Database {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(file_name)
            .unwrap();
        Database { file }
    }

    pub fn add_record(&mut self, record: &Record) {
        let line = format!("{},{}\n", record.id, record.content);
        writeln!(self.file, "{}", line).unwrap();
        println!("📃 Item added: {}", record.content);
    }
}
