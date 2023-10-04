use std::fmt::Display;
use std::str::FromStr;

struct Password(String);

#[derive(Debug)]
pub enum PasswordParseError {
    InvalidFormat,
}

impl Display for PasswordParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasswordParseError::InvalidFormat => {
                write!(f, "Password length must be greater than 5")
            }
        }
    }
}

impl FromStr for Password {
    type Err = PasswordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 6 {
            Err(PasswordParseError::InvalidFormat)
        } else {
            Ok(Password(s.to_string()))
        }
    }
}

impl std::error::Error for PasswordParseError {}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "**************************")
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let unsecure_password: String = "xzs".to_string();

    let secure_password: Password = Password::from_str(&unsecure_password)?;

    println!("unsecure_password: {}", unsecure_password);
    println!("secure_password: {}", secure_password);

    Ok(())
}
