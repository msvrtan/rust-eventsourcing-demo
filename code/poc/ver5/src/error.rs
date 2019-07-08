use core::fmt;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum BankAccountError {
    NoState,
}

impl Error for BankAccountError {}

impl fmt::Display for BankAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BankAccountError: Oh no, something bad went down")
    }
}
