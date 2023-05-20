use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::types::contact::{Contact, ContactId};

#[derive(Clone)]
pub struct Store {
    pub contacts: Arc<RwLock<HashMap<ContactId, Contact>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            contacts: Arc::new(RwLock::new(Self::init())),
        }
    }
    fn init() -> HashMap<ContactId, Contact> {
        let file = include_str!("../contacts.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}
