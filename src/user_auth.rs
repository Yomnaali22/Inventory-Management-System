use crate::handle_env_variables;
use crate::parse_data::JsonData;
use serde::{Deserialize, Serialize};
use std::io::{Write, stdin, stdout};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub password: String,
}

pub fn authenticate(username: &str, password: &str) -> Result<User, String> {
    let path = handle_env_variables("USERS_JSON_PATH");
    let users: Vec<User> = match JsonData::process(&path) {
        Ok(users) => users,
        Err(err) => return Err(err.to_string()),
    };
    let valid = users
        .iter()
        .any(|user| user.name == username && user.password == password);

    if valid {
        Ok(User {
            name: username.to_string(),
            password: password.to_string(),
        })
    } else {
        Err(format!("Invalid username or password. Please try again."))
    }
}

pub fn process_user_input() -> Result<User, String> {
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
