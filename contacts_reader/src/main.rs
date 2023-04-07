use std::{error::Error, io, process};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    phone: usize,
}


fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
