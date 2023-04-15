use std::{env, error::Error, io, process};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Contact {
    id: usize,
    name: String,
    surname: String,
    nickname: String,
    phone: Option<usize>,
    phone_description: String,
    category: String,
    address: String,
    email: String,
    url: String,
    facebook_url: String,
    twitter_handle: String,
    note: String,
}

impl Contact {
    fn new(
        id: &str,
        name: &str,
        surname: &str,
        nickname: &str,
        phone: &str,
        phone_description: &str,
        category: &str,
        address: &str,
        email: &str,
        url: &str,
        facebook_url: &str,
        twitter_handle: &str,
        note: &str,
    ) -> Self {
        let phone_str = phone.to_string();
        let phone: Option<usize>;
        if phone_str.is_empty() {
            phone = None;
        } else {
            phone = Some(phone_str.parse::<usize>().unwrap());
        }
        Contact {
            id: id.to_string().parse::<usize>().unwrap(),
            name: name.to_string(),
            surname: surname.to_string(),
            nickname: nickname.to_string(),
            phone,
            phone_description: phone_description.to_string(),
            category: category.to_string(),
            address: address.to_string(),
            email: email.to_string(),
            url: url.to_string(),
            facebook_url: facebook_url.to_string(),
            twitter_handle: twitter_handle.to_string(),
            note: note.to_string(),
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
    if env::args().len() == 1 {
        println!("Init interactive mode");
        loop {
            let mut rdr = get_reader_from_file()?;
            let query = get_user_input();
            search_and_show(query, &mut rdr)?;
        }
    } else {
        let mut rdr = get_reader_from_file()?;
        let query = match env::args().nth(1) {
            None => return Err(From::from("expected 1 argument, but got none")),
            Some(query) => query,
        };
        search_and_show(query, &mut rdr)?;
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

fn get_reader_from_file() -> Result<csv::Reader<Box<std::fs::File>>, Box<dyn Error>> {
    let filename = "contacts.csv";
    let f = std::fs::File::open(filename)?;
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(Box::new(f));
    Ok(rdr)
}

fn search_and_show(
    query: String,
    rdr: &mut csv::Reader<Box<std::fs::File>>,
) -> Result<(), Box<dyn Error>> {
    println!("Results of `{:?}`:", query);
    let mut record = csv::StringRecord::new();
    while rdr.read_record(&mut record)? {
        if record
            .iter()
            .any(|field| field.to_lowercase().contains(&query.to_lowercase()))
        {
            let contact = Contact::new(
                &record[0],
                &record[1],
                &record[2],
                &record[3],
                &record[4],
                &record[5],
                &record[6],
                &record[7],
                &record[8],
                &record[9],
                &record[10],
                &record[11],
                &record[12],
            );
            let phone = match contact.phone {
                Some(phone) => phone.to_string(),
                None => "".to_string(),
            };
            println!(
                "{} {} - {} {} {} {}",
                phone,
                contact.phone_description,
                contact.name,
                contact.surname,
                contact.nickname,
                contact.category
            );
        }
    }
    Ok(())
}
