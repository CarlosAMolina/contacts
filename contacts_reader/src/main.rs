use std::{env, error::Error, io, process};

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
        if record.iter().any(|field| field.to_lowercase().contains(&query.to_lowercase())) {
            println!("{:?}", record);
        }
    }
    Ok(())
}
