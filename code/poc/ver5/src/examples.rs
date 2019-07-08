use crate::error::BankAccountError;
use crate::event::DomainMessage;
use crate::model::{BankAccountAggregate, BankAccountId, CustomerId};

pub fn examples() {
    match example_open_bank_account() {
        Ok(_) => println!("Bank account opened"),
        Err(_) => panic!("Bank account not opened"),
    }

    println!("Done ver5!");
}
pub fn example_open_bank_account() -> Result<(), ()> {
    Ok(())
}

/// Create a new to-do item
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenBankAccount {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
}

impl OpenBankAccount {
    pub fn new(id: BankAccountId, customer_id: CustomerId) -> OpenBankAccount {
        OpenBankAccount { id, customer_id }
    }
}

pub struct OpenBankAccountHandler {
    repository: BankAccountRepository,
}

impl OpenBankAccountHandler {
    pub fn new(repository: BankAccountRepository) -> OpenBankAccountHandler {
        OpenBankAccountHandler { repository }
    }

    pub fn handle(&self, cmd: OpenBankAccount) -> Result<(), BankAccountError> {
        // Create aggregate
        let mut agg = BankAccountAggregate::new();

        agg.open_acc(cmd.id, cmd.customer_id)?;
        // Get events
        let events = agg.get_new_events();

        println!("{:?}", &events);

        // Store events
        self.repository.save(events.to_vec())
    }
}

///
/// ===========================================================================
///

pub struct BankAccountRepository {}

impl BankAccountRepository {
    pub fn save(&self, _events: Vec<DomainMessage>) -> Result<(), BankAccountError> {
        Ok(())
    }
}
