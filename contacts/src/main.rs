use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, error::Error, io, process};

use csv::StringRecord;
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

    fn new_from_csv_record(record: &StringRecord) -> Contact {
        Contact::new(
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
        )
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    if env::args().len() == 3 {
        let arg_1 = env::args().nth(1).unwrap().to_lowercase();
        if arg_1 == "export" {
            let id = env::args().nth(2).unwrap().parse::<usize>().unwrap();
            println!("Init export contact ID {}", id);
            let rdr = get_reader_from_file()?;
            match search_id(id, rdr)? {
                Some(contact) => {
                    let html = get_html(contact);
                    write_to_file(html)?;
                }
                None => println!("Contact not found"),
            };
        } else {
            println!("Invalid argument `{}`", arg_1);
        }
    } else if env::args().len() == 2 {
        let mut rdr = get_reader_from_file()?;
        let query = env::args().nth(1).unwrap();
        search_and_show(query, &mut rdr)?;
    } else if env::args().len() == 1 {
        println!("Init interactive mode");
        loop {
            let mut rdr = get_reader_from_file()?;
            let query = get_user_input();
            search_and_show(query, &mut rdr)?;
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
    println!("Results of `{}`:", query);
    let mut record = csv::StringRecord::new();
    while rdr.read_record(&mut record)? {
        if record
            .iter()
            .any(|field| field.to_lowercase().contains(&query.to_lowercase()))
        {
            let contact = Contact::new_from_csv_record(&record);
            let phone = match contact.phone {
                Some(phone) => phone.to_string(),
                None => "".to_string(),
            };
            println!(
                "{} {} - {} {} {} {}. ID {}",
                phone,
                contact.phone_description,
                contact.name,
                contact.surname,
                contact.nickname,
                contact.category,
                contact.id
            );
        }
    }
    Ok(())
}

fn search_id(
    id: usize,
    mut rdr: csv::Reader<Box<std::fs::File>>,
) -> Result<Option<Contact>, Box<dyn Error>> {
    for record in rdr.deserialize() {
        let contact: Contact = record?;
        if contact.id == id {
            return Ok(Some(contact));
        }
    }
    Ok(None)
}

fn get_html(contact: Contact) -> String {
    let mut result = String::new();
    let phone = match contact.phone {
        Some(phone) => phone.to_string(),
        None => "".to_string(),
    };
    match get_image_path_name(contact.id) {
        Some(image_path_name) => result.push_str(&format!(
            "<img src=\"{}\" alt=\"{}\" width=\"{}\" height=\"{}\">",
            image_path_name,
            "Profile photo".to_string(),
            200,
            200
        )),
        None => println!("No image found"),
    };
    result.push_str("\n");
    result.push_str(&get_html_tag_p(format!("ID: {}", contact.id)));
    result.push_str("\n");
    result.push_str(&get_html_tag_p(format!("Name: {}", contact.name)));
    result.push_str("\n");
    result.push_str(&get_html_tag_p(format!("Surname: {}", contact.surname)));
    result.push_str("\n");
    result.push_str(&get_html_tag_p(format!("Nickname: {}", contact.nickname)));
    result.push_str("\n");
    result.push_str(&get_html_tag_p(format!("Category: {}", contact.category)));
    result.push_str("\n");
    result.push_str(&get_html_tag_p(format!("Phone: {}", phone)));
    result.push_str("\n");
    result.push_str(&get_html_tag_p(format!(
        "Phone description: {}",
        contact.phone_description
    )));
    result.push_str("\n");
    result.push_str(&get_html_tag_p(format!("Address: {}", contact.address)));
    result.push_str("\n");
    result.push_str(&get_html_tag_p(format!("Email: {}", contact.email)));
    result.push_str("\n");
    result.push_str(&get_html_tag_p(format!("Url: {}", contact.url)));
    result.push_str("\n");
    result.push_str(&get_html_tag_p(format!(
        "Facebook: {}",
        contact.facebook_url
    )));
    result.push_str("\n");
    result.push_str(&get_html_tag_p(format!(
        "Twitter: {}",
        contact.twitter_handle
    )));
    result.push_str("\n");
    result.push_str(&get_html_tag_p(format!("Note: {}", contact.note)));
    result
}

fn get_image_path_name(contact_id: usize) -> Option<String> {
    let images_path = Path::new("images");
    if let Ok(entries) = fs::read_dir(images_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let contact_path_name = images_path
                    .join(format!("{}-", contact_id))
                    .to_string_lossy()
                    .to_string();
                let entry_path_name = entry.path().to_string_lossy().to_string();
                if entry_path_name.starts_with(&contact_path_name) {
                    return Some(entry_path_name);
                }
            }
        }
    }
    None
}

fn get_html_tag_p(string: String) -> String {
    format!("<p>{}</p>", string)
}

fn write_to_file(text_to_write_all: String) -> Result<(), Box<dyn Error>> {
    let file_path_name = "contact.html";
    println!("Init export to {}", file_path_name);
    let path = Path::new(&file_path_name);
    let mut file = File::create(path)?;
    file.write_all(text_to_write_all.as_bytes())?;
    Ok(())
}