use crate::parse_data::JsonData;
use serde::{Deserialize, Serialize};
use std::io::{Write, stdin, stdout};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    name: String,
    password: String,
}
fn authenticate(username: &str, password: &str) -> Result<String, String> {
    let users: Vec<User> = match JsonData::process("users.json") {
        Ok(users) => users,
        Err(err) => return Err(err.to_string()),
    };
    let valid = users
        .iter()
        .any(|user| user.name == username && user.password == password);

    if valid {
        Ok(String::from("Login successful!"))
    } else {
        Err(String::from("Invalid username or password"))
    }
}

pub fn process_user_input() -> Result<String, String> {
    print!("Enter username: ");
    stdout().flush().unwrap();
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    print!("Enter password: ");
    stdout().flush().unwrap();
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    authenticate(username, password)
}
