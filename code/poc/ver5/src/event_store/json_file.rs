extern crate serde;
extern crate serde_json;

use crate::prelude::{
    BankAccountEvent, BankAccountEventStore, BankAccountEventStoreError, BankAccountId,
};
use std::fs;
use std::sync::Mutex;

type GetEventsResult = Result<Vec<BankAccountEvent>, BankAccountEventStoreError>;
type SaveEventsResult = Result<(), BankAccountEventStoreError>;

pub struct JsonFileBankAccountEventStore {
    pub events: Mutex<Vec<BankAccountEvent>>,
    pub file_path: String,
}

impl JsonFileBankAccountEventStore {
    pub fn new(file_path: String) -> JsonFileBankAccountEventStore {
        let events = Self::load_data(&file_path).unwrap();

        JsonFileBankAccountEventStore {
            events: Mutex::new(events),
            file_path: file_path,
        }
    }

    fn load_data(file_path: &String) -> Result<Vec<BankAccountEvent>, BankAccountEventStoreError> {
        let content = match fs::read_to_string(file_path) {
            Ok(data) => data,
            Err(_) => "[]".to_string(),
        };

        let events: Vec<BankAccountEvent> = serde_json::from_str(&content)?;

        Ok(events)
    }
    fn save_data(&self) -> Result<(), BankAccountEventStoreError> {
        let m_entities = self.events.lock().unwrap();
        let mut values = Vec::new();
        for value in m_entities.iter() {
            values.push((*value).clone());
        }

        let content = ::serde_json::to_string_pretty(&values).unwrap();

        fs::write(&self.file_path, content).expect("Something went wrong writing the file");

        Ok(())
    }

    fn store_data(&self, events: Vec<BankAccountEvent>) -> SaveEventsResult {
        let mut m_entities = self.events.lock().unwrap();

        for event in events {
            m_entities.push(event.clone());
        }

        Ok(())
    }

    fn purge_events(&self) -> Result<(), BankAccountEventStoreError> {
        let mut m_entities = self.events.lock().unwrap();

        m_entities.clear();

        Ok(())
    }
    fn purge_file(&self) -> Result<(), BankAccountEventStoreError> {
        self.save_data()
    }
    pub fn purge_all(&self) -> Result<(), BankAccountEventStoreError> {
        self.purge_events()?;
        self.purge_file()?;

        Ok(())
    }
}

impl BankAccountEventStore for JsonFileBankAccountEventStore {
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
        self.store_data(events)?;

        self.save_data()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{BankAccountEvent, BankAccountEventStore, JsonFileBankAccountEventStore};

    fn get_event_store(file_name: String) -> JsonFileBankAccountEventStore {
        let file_path = get_path(file_name);
        JsonFileBankAccountEventStore::new(file_path)
    }

    fn get_empty_event_store(file_name: String) -> JsonFileBankAccountEventStore {
        let file_path = get_path(file_name);
        let event_store = JsonFileBankAccountEventStore::new(file_path);
        match event_store.purge_all() {
            Ok(_) => event_store,
            _ => panic!("Purging failed"),
        }
    }

    #[test]
    fn store_first_event() {
        // Arrange
        let events = vec![BankAccountEvent::acc_opened(100, 20)];

        // Act
        let event_store = get_empty_event_store("test_storing_1st_event".to_string());
        let result = event_store.save_events(events);

        // Assert
        assert_eq!(Ok(()), result);

        let stored_events = event_store.get_events(100).unwrap();
        assert_eq!(1, stored_events.len());
    }

    #[test]
    fn store_two_events() {
        // Arrange
        let file_name = "test_storing_two_events".to_string();
        let events = vec![BankAccountEvent::acc_opened(100, 20)];
        let events2 = vec![BankAccountEvent::acc_opened(101, 20)];

        let event_store = get_empty_event_store(file_name);
        event_store.save_events(events).unwrap();
        event_store.save_events(events2).unwrap();

        let stored_events1 = event_store.get_events(100).unwrap();
        assert_eq!(1, stored_events1.len());
        let stored_events2 = event_store.get_events(101).unwrap();
        assert_eq!(1, stored_events2.len());
    }

    #[test]
    fn store_two_event_streams() {
        // Arrange
        let file_name = "test_storing_two_event_streams".to_string();
        let events = vec![
            BankAccountEvent::acc_opened(100, 20),
            BankAccountEvent::acc_opened(101, 20),
        ];
        let events2 = vec![
            BankAccountEvent::acc_opened(200, 20),
            BankAccountEvent::acc_opened(201, 20),
        ];

        let event_store = get_empty_event_store(file_name);
        event_store.save_events(events).unwrap();
        event_store.save_events(events2).unwrap();

        let stored_events1 = event_store.get_events(100).unwrap();
        assert_eq!(1, stored_events1.len());
        let stored_events2 = event_store.get_events(101).unwrap();
        assert_eq!(1, stored_events2.len());
        let stored_events2 = event_store.get_events(200).unwrap();
        assert_eq!(1, stored_events2.len());
        let stored_events2 = event_store.get_events(201).unwrap();
        assert_eq!(1, stored_events2.len());
    }

    #[test]
    fn store_second_event() {
        // Arrange
        let events = vec![BankAccountEvent::acc_opened(100, 20)];
        let events2 = vec![BankAccountEvent::acc_opened(101, 20)];

        {
            let event_store = get_empty_event_store("test_storing_2nd_event".to_string());
            event_store.save_events(events).unwrap();
        }

        let event_store2 = get_event_store("test_storing_2nd_event".to_string());
        event_store2.save_events(events2).unwrap();

        let stored_events1 = event_store2.get_events(100).unwrap();
        assert_eq!(1, stored_events1.len());
        let stored_events2 = event_store2.get_events(101).unwrap();
        assert_eq!(1, stored_events2.len());
    }

    fn get_path(file_name: String) -> String {
        let path = "/work/noob/rust-eventsourcing-demo/dev/code/poc/ver5/tmp/".to_string();
        let file_path = format!("{}{}.json", path, file_name);

        file_path
    }
}
