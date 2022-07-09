use chrono;
use std::io::{Read, Write};

enum Operation {
    New,
    Show,
    Reset,
}

fn main() {
    let operation = match std::env::args().nth(1) {
        Some(operation) => {
            if operation == "new" {
                Operation::New
            } else if operation == "show" {
                Operation::Show
            } else if operation == "reset" {
                Operation::Reset
            } else {
                panic!("Unknown operation: {}", operation);
            }
        }
        None => {
            eprintln!(
                "Usage: {} new <message> (appends the note file with a new note)",
                std::env::args().nth(0).unwrap()
            );
            eprintln!(
                "Usage: {} show (shows the notes file)",
                std::env::args().nth(0).unwrap()
            );
            eprintln!(
                "Usage: {} reset (deletes the notes file)",
                std::env::args().nth(0).unwrap()
            );
            std::process::exit(1);
        }
    };

    match operation {
        Operation::New => {
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
                    println!("Note added");
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
        Operation::Reset => match std::fs::remove_file("notes.txt") {
            Ok(_) => {
                println!("Notes reset");
            }
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        },
    }
}
