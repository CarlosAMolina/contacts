use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, error::Error};

use crate::contact;

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
