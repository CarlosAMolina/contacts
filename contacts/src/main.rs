use std::process;

mod cli;
mod contact;
mod file_csv;
mod html;

fn main() {
    if let Err(err) = cli::run() {
        println!("{}", err);
        process::exit(1);
    }
}

