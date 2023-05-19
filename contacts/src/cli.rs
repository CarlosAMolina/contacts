use std::{env, error::Error, io};

use crate::file_csv;
use crate::html;

pub fn run() -> Result<(), Box<dyn Error>> {
    if env::args().len() == 3 {
        let arg_1 = env::args().nth(1).unwrap().to_lowercase();
        if arg_1 == "export" {
            let id = env::args().nth(2).unwrap().parse::<usize>().unwrap();
            println!("Init export contact ID {}", id);
            let rdr = file_csv::get_reader_from_file()?;
            match file_csv::search_id(id, rdr)? {
                Some(contact) => {
                    let html = html::get_html(contact);
                    html::write_to_file(html)?;
                }
                None => println!("Contact not found"),
            };
        } else {
            println!("Invalid argument `{}`", arg_1);
        }
    } else if env::args().len() == 2 {
        let mut rdr = file_csv::get_reader_from_file()?;
        let query = env::args().nth(1).unwrap();
        file_csv::search_and_show(query, &mut rdr)?;
    } else if env::args().len() == 1 {
        println!("Init interactive mode");
        loop {
            let mut rdr = file_csv::get_reader_from_file()?;
            let query = get_user_input();
            file_csv::search_and_show(query, &mut rdr)?;
        }
    } else {
        println!("Invalid number of arguments");
    }
    Ok(())
}

fn get_user_input() -> String {
    let mut input = String::new();
    println!("Type a search term and press Enter");
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");
        input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error reading user input");
                continue;
            }
        };
        if input.is_empty() {
            continue;
        }
        break;
    }
    input
}
