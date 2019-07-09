use crate::error::BankAccountAppError;
use bankaccount_core::event::DomainMessage;

pub trait BankAccountRepository {
    fn save(self, messages: Vec<DomainMessage>) -> Result<(), BankAccountAppError>;
}

pub struct InMemoryBankAccountRepository {}

impl BankAccountRepository for InMemoryBankAccountRepository {
    fn save(self, messages: Vec<DomainMessage>) -> Result<(), BankAccountAppError> {
        unimplemented!()
    }
}
