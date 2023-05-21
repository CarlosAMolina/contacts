use std::collections::HashMap;
use std::error::Error;

use crate::types::contact;

pub fn get_all_contacts() -> Result<HashMap<contact::ContactId, contact::Contact>, Box<dyn Error>> {
    let mut result: HashMap<contact::ContactId, contact::Contact> = HashMap::new();
    let mut rdr = get_reader_from_file()?;
    for record in rdr.deserialize() {
        let contact_: contact::Contact = record?;
        result.insert(contact::ContactId(contact_.id.to_string()), contact_);
    }
    Ok(result)
}

fn get_reader_from_file() -> Result<csv::Reader<Box<std::fs::File>>, Box<dyn Error>> {
    let filename = "contacts.csv";
    let f = std::fs::File::open(filename)?;
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(Box::new(f));
    Ok(rdr)
}
