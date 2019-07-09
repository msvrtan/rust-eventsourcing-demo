use crate::error::BankAccountAppError;
use bankaccount_core::event::DomainMessage;
use bankaccount_core::model::BankAccountAggregate;
use bankaccount_core::BankAccountId;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BankAccountRepository {}

impl BankAccountRepository {
    pub fn save(self, _messages: Vec<DomainMessage>) -> Result<(), BankAccountAppError> {
        Err(BankAccountAppError::RepositoryError)
    }

    pub fn load(self, _id: BankAccountId) -> Result<BankAccountAggregate, BankAccountAppError> {
        Err(BankAccountAppError::RepositoryError)
    }
}
