use bankaccount_core::error::BankAccountError;
use core::fmt;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum BankAccountAppError {
    DomainError,
    RepositoryError,
}

impl Error for BankAccountAppError {}

impl fmt::Display for BankAccountAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BankAccountAppError: Oh no, something bad went down")
    }
}
impl From<BankAccountError> for BankAccountAppError {
    fn from(_error: BankAccountError) -> Self {
        BankAccountAppError::DomainError
    }
}
