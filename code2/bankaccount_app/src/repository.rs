use crate::error::BankAccountAppError;
use bankaccount_core::event::{BankAccountEvent, DomainMessage};
use bankaccount_core::model::BankAccountAggregate;
use bankaccount_core::BankAccountId;
use std::sync::Mutex;

#[derive(Debug)]
pub struct BankAccountRepository {
    event_store: InMemoryEventStore,
}

impl BankAccountRepository {
    pub fn new(event_store: InMemoryEventStore) -> Self {
        BankAccountRepository { event_store }
    }
    pub fn save(self, messages: Vec<DomainMessage>) -> Result<(), BankAccountAppError> {
        self.event_store.save(messages)
    }

    pub fn load(self, id: BankAccountId) -> Result<BankAccountAggregate, BankAccountAppError> {
        self.event_store.load(id)
    }
}

trait EventStore {}

#[derive(Debug)]
pub struct InMemoryEventStore {
    pub events: Mutex<Vec<BankAccountEvent>>,
}

impl EventStore for InMemoryEventStore {}

impl InMemoryEventStore {
    pub fn new() -> InMemoryEventStore {
        InMemoryEventStore {
            events: Mutex::new(Vec::new()),
        }
    }

    pub fn save(self, messages: Vec<DomainMessage>) -> Result<(), BankAccountAppError> {
        Err(BankAccountAppError::RepositoryError)
    }

    pub fn load(self, id: BankAccountId) -> Result<BankAccountAggregate, BankAccountAppError> {
        Err(BankAccountAppError::RepositoryError)
    }
}
