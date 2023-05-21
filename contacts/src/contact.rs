use crate::types::contact;

impl contact::Contact {
    pub fn contains(&self, value: &String) -> bool {
        let value = value.to_lowercase();
        let phone = match self.phone {
            Some(x) => x.to_string(),
            None => "".to_string(),
        };
        let fields_values: Vec<&String> = vec![
            &self.name,
            &self.surname,
            &self.nickname,
            &phone,
            &self.phone_description,
            &self.category,
            &self.address,
            &self.email,
            &self.url,
            &self.facebook_url,
            &self.twitter_handle,
            &self.note,
        ];
        for field_value in fields_values.iter() {
            if field_value.to_lowercase().contains(&value) {
                return true;
            }
        }
        false
    }
}
