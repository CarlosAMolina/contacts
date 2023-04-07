use std::{env, error::Error, io, process};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Contact {
    name: String,
    phone: usize,
}

impl Contact {
    fn new(name: &str, phone: &str) -> Self {
        Contact {
            name: name.to_string(),
            phone: phone.to_string().parse::<usize>().unwrap(),
        }
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let query = match env::args().nth(1) {
        None => return Err(From::from("expected 1 argument, but got none")),
        Some(query) => query,
    };

    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut record = csv::StringRecord::new();

    while rdr.read_record(&mut record)? {
        if record
            .iter()
            .any(|field| field.to_lowercase().contains(&query.to_lowercase()))
        {
            let contact = Contact::new(&record[0], &record[1]);
            println!("{:?} - {:?}", contact.phone, contact.name);
        }
    }
    Ok(())
}
