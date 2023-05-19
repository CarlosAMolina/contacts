use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, error::Error};

use crate::contact;

pub fn get_html(contact: contact::Contact) -> String {
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
            image_path_name, "Profile photo", 200, 200
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
        for entry in entries.flatten() {
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
    None
}

fn set_value_to_html(html: &mut String, value: String, title: &str) {
    if !value.is_empty() {
        html.push('\n');
        html.push_str(&get_html_for_value(value, title));
    }
}

fn get_html_for_value(value: String, title: &str) -> String {
    let mut result = String::new();
    result.push_str(&get_html_title_tag_p(title));
    result.push('\n');
    result.push_str(&get_html_tag_p(value));
    result
}

fn get_html_title_tag_p(string: &str) -> String {
    format!(r#"<p class="title">{}</p>"#, string)
}

fn get_html_tag_p(string: String) -> String {
    format!("<p>{}</p>", string)
}

pub fn write_to_file(text_to_write_all: String) -> Result<(), Box<dyn Error>> {
    let path_name = env::var("CONTACT_HTML_PATH_NAME").unwrap_or("/tmp".to_string());
    let file_name = "contact.html";
    let file_path = Path::new(&path_name).join(file_name);
    println!("Init export to {}", file_path.display());
    let mut file = File::create(file_path)?;
    file.write_all(text_to_write_all.as_bytes())?;
    Ok(())
}
