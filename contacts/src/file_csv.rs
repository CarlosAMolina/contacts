use std::error::Error;

use crate::types::contact;

pub fn get_reader_from_file() -> Result<csv::Reader<Box<std::fs::File>>, Box<dyn Error>> {
    let filename = "contacts.csv";
    let f = std::fs::File::open(filename)?;
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(Box::new(f));
    Ok(rdr)
}

pub fn search_and_show(
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

pub fn search_id(
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
