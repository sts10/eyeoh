// pub mod eyeoh;

use std::io;

pub fn gets() -> Result<String, String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim().to_string()),
        Err(error) => Err(format!("Error: {}", error.to_string())),
    }
}
