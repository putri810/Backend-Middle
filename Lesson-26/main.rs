use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum UserError {
    NotFound,
    InvalidInput(String),
    CustomError(String),
}

impl Error for UserError {}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserError::NotFound => write!(f, "User not found"),
            UserError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            UserError::CustomError(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}

fn find_user(id: u32) -> Result<String, UserError> {
    if id == 42 {
        Ok(String::from("Alice"))
    } else {
        Err(UserError::NotFound)
    }
}

fn process_user(username: &str) -> Result<(), UserError> {
    if username.chars().count() < 5 {
        Err(UserError::InvalidInput(String::from("Username too short")))
    } else {
        println!("Processing user: {}", username);
        Ok(())
    }
}

fn main() {
    let user_id = 42;
    match find_user(user_id) {
        Ok(username) => {
            if let Err(err) = process_user(&username) {
                println!("Error processing user: {}", err);
            } else {
                println!("User processing complete");
            }
        }
        Err(err) => println!("Error finding user: {}", err),
    }
}
