use chrono;
use std::io::{Read, Write};

enum Operation {
    New,
    Show,
}

fn main() {
    let operation = match std::env::args().nth(1) {
        Some(operation) => {
            if operation == "new" {
                Operation::New
            } else if operation == "show" {
                Operation::Show
            } else {
                panic!("Unknown operation: {}", operation);
            }
        }
        None => {
            eprintln!("Usage: {} <operation>", std::env::args().nth(0).unwrap());
            std::process::exit(1);
        }
    };

    match operation {
        Operation::New => {
            // create a file named notes if it doesn't exist
            match std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("notes.txt")
            {
                Ok(mut file) => {
                    let mut note = chrono::Local::now().to_rfc2822().to_string();
                    note.push_str(" :: ");
                    let remaining_args = std::env::args().skip(2).collect::<Vec<_>>();
                    note.push_str(&remaining_args.join(" "));
                    note.push_str("\n");
                    file.write_all(note.as_bytes()).unwrap();
                }

                Err(error) => {
                    eprintln!("Error: {}", error);
                    std::process::exit(1);
                }
            }
        }
        Operation::Show => match std::fs::File::open("notes.txt") {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                println!("{}", contents);
            }

            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        },
    }
}
