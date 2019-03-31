use crate::prelude::{
    BankAccountEvent, BankAccountEventStore, BankAccountEventStoreError, BankAccountId,
};
use std::sync::Mutex;

type GetEventsResult = Result<Vec<BankAccountEvent>, BankAccountEventStoreError>;
type SaveEventsResult = Result<(), BankAccountEventStoreError>;

pub struct InMemoryBankAccountEventStore {
    pub events: Mutex<Vec<BankAccountEvent>>,
}

impl InMemoryBankAccountEventStore {
    pub fn new() -> InMemoryBankAccountEventStore {
        InMemoryBankAccountEventStore {
            events: Mutex::new(Vec::new()),
        }
    }
}

impl BankAccountEventStore for InMemoryBankAccountEventStore {
    fn get_events(&self, id: BankAccountId) -> GetEventsResult {
        let m_entities = self.events.lock().unwrap();
        let mut values = Vec::new();
        for value in m_entities.iter() {
            if id == value.get_aggregate_id() {
                values.push((*value).clone());
            }
        }

        Ok(values)
    }
    fn save_events(&self, events: Vec<BankAccountEvent>) -> SaveEventsResult {
        let mut m_entities = self.events.lock().unwrap();

        for event in events {
            m_entities.push(event.clone());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{BankAccountEvent, BankAccountEventStore, InMemoryBankAccountEventStore};

    #[test]
    fn check_get_events_returns_only_events_with_expected_id() {
        // Arrange
        let event_store = InMemoryBankAccountEventStore::new();

        let events = vec![
            BankAccountEvent::acc_opened(100, 20),
            BankAccountEvent::acc_opened(101, 20),
        ];
        let expected = vec![BankAccountEvent::acc_opened(100, 20)];

        match event_store.save_events(events) {
            Ok(_) => println!("Events saved"),
            Err(_) => panic!("Cant save events"),
        }

        // Act
        let result = event_store.get_events(100).unwrap();

        // Assert
        assert_eq!(expected, result);
    }
}
