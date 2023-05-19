use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, error::Error, io, process};

mod contact;

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
            let contact = contact::Contact::new_from_csv_record(&record);
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
) -> Result<Option<contact::Contact>, Box<dyn Error>> {
    for record in rdr.deserialize() {
        let contact: contact::Contact = record?;
        if contact.id == id {
            return Ok(Some(contact));
        }
    }
    Ok(None)
}

fn get_html(contact: contact::Contact) -> String {
    let mut result = String::new();
    let phone = match contact.phone {
        Some(phone) => phone.to_string(),
        None => "".to_string(),
    };
    result.push_str(
        r#"<!DOCTYPE html>
<html lang="es">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Contact</title>
    <style>
        body {
            background-color: #1a1a1a;
        }
        .title   {
            color: #ce8989;
            font-size: 12px;
        }
        p    {
            color: #ffffff;
            font-size: 16px;
        }
    </style>
  </head>
  <body>"#,
    );
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
    set_value_to_html(&mut result, contact.id.to_string(), "ID");
    set_value_to_html(&mut result, contact.name, "Name");
    set_value_to_html(&mut result, contact.surname, "Surname");
    set_value_to_html(&mut result, contact.nickname, "Nickname");
    set_value_to_html(&mut result, contact.category, "Category");
    set_value_to_html(&mut result, phone, "Phone");
    set_value_to_html(&mut result, contact.phone_description, "Phone description");
    set_value_to_html(&mut result, contact.address, "Address");
    set_value_to_html(&mut result, contact.email, "Email");
    set_value_to_html(&mut result, contact.url, "Url");
    set_value_to_html(&mut result, contact.facebook_url, "Facebook");
    set_value_to_html(&mut result, contact.twitter_handle, "Twitter");
    set_value_to_html(&mut result, contact.note, "Note");
    result.push_str(
        "
  </body>
</html>",
    );
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

fn set_value_to_html(html: &mut String, value: String, title: &str) {
    if !value.is_empty() {
        html.push_str("\n");
        html.push_str(&get_html_for_value(value, title));
    }
}

fn get_html_for_value(value: String, title: &str) -> String {
    let mut result = String::new();
    result.push_str(&get_html_title_tag_p(title));
    result.push_str("\n");
    result.push_str(&get_html_tag_p(value));
    result
}

fn get_html_title_tag_p(string: &str) -> String {
    format!(r#"<p class="title">{}</p>"#, string)
}

fn get_html_tag_p(string: String) -> String {
    format!("<p>{}</p>", string)
}

fn write_to_file(text_to_write_all: String) -> Result<(), Box<dyn Error>> {
    let path_name = env::var("CONTACT_HTML_PATH_NAME").unwrap_or("/tmp".to_string());
    let file_name = "contact.html";
    let file_path = Path::new(&path_name).join(file_name);
    println!("Init export to {}", file_path.display());
    let mut file = File::create(file_path)?;
    file.write_all(text_to_write_all.as_bytes())?;
    Ok(())
}
