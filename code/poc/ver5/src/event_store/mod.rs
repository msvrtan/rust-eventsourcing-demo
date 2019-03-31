pub mod in_memory;
pub mod json_file;
use crate::event::BankAccountEvent;
use crate::model::BankAccountError;
use crate::model::BankAccountId;
use std::{error::Error, fmt};

type GetEventsResult = Result<Vec<BankAccountEvent>, BankAccountEventStoreError>;
type SaveEventsResult = Result<(), BankAccountEventStoreError>;

pub trait BankAccountEventStore {
    fn get_events(&self, id: BankAccountId) -> GetEventsResult;
    fn save_events(&self, events: Vec<BankAccountEvent>) -> SaveEventsResult;
}

#[derive(Debug, PartialEq)]
pub enum BankAccountEventStoreError {
    TestFailed,
}

impl Error for BankAccountEventStoreError {}

impl fmt::Display for BankAccountEventStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BankAccountEventStoreError: :(")
    }
}

impl From<BankAccountEventStoreError> for BankAccountError {
    fn from(_err: BankAccountEventStoreError) -> BankAccountError {
        BankAccountError::CantSaveEvent
    }
}
