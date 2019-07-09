use crate::command::{Deposit, OpenBankAccount, Withdraw};
use crate::error::BankAccountAppError;
use crate::repository::BankAccountRepository;
use bankaccount_core::event::DomainMessage;
use bankaccount_core::model::BankAccountAggregate;
use bankaccount_core::BankAccountId;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, RwLock};

type HandlerResult = Result<(), BankAccountAppError>;

struct OpenBankAccountHandler {
    repository: BankAccountRepository,
}

impl OpenBankAccountHandler {
    fn new(repository: BankAccountRepository) -> Self {
        Self { repository }
    }

    fn handle(self, command: OpenBankAccount) -> HandlerResult {
        let mut agg = BankAccountAggregate::new();

        agg.open_acc(command.id, command.customer_id)?;

        let events = agg.get_new_events();

        self.repository.save(events.to_vec());

        Ok(())
    }
}

//
//
//
//
// #################################################################################################
//
//
//
//

#[derive(Debug)]
struct DepositHandler {
    repository: Arc<RwLock<BankAccountRepository>>,
}

impl DepositHandler {
    fn new(repository: BankAccountRepository) -> Self {
        Self {
            repository: Arc::new(RwLock::new(repository)),
        }
    }
    fn handle(&mut self, command: Deposit) -> HandlerResult {
        let mut agg = self.load(command.id)?;

        agg.deposit(command.amount);

        let events = agg.get_new_events();

        self.save(events.to_vec());

        Ok(())
    }

    fn load(&mut self, id: BankAccountId) -> Result<BankAccountAggregate, BankAccountAppError> {
        self.repository.read().unwrap().load(id)
    }
    fn save(&mut self, events: Vec<DomainMessage>) -> Result<(), BankAccountAppError> {
        self.repository.write().unwrap().save(events.to_vec());

        Ok(())
    }
}

//
//
//
//
// #################################################################################################
//
//
//
//

struct WithdrawHandler {
    repository: BankAccountRepository,
}

impl WithdrawHandler {
    fn new(repository: BankAccountRepository) -> Self {
        Self { repository }
    }
    fn handle(self, command: Withdraw) -> HandlerResult {
        /*
        let mut agg = self.repository.load(command.id)?;

        agg.withdraw(command.amount);

        let events = agg.get_new_events();

        self.repository.save(events.to_vec());
        */

        Ok(())
    }
}

//
//
//
//
// #################################################################################################
//
//
//
//

#[cfg(test)]
mod tests {
    use crate::command::{Deposit, OpenBankAccount, Withdraw};
    use crate::error::BankAccountAppError;
    use crate::handler::{DepositHandler, OpenBankAccountHandler, WithdrawHandler};
    use crate::repository::{BankAccountRepository, InMemoryEventStore};
    use bankaccount_core::{BankAccountId, CustomerId};

    type TestResult = Result<(), BankAccountAppError>;

    const ACCOUNT_ID: BankAccountId = 123;
    const CUSTOMER_ID: CustomerId = 5000;

    #[test]
    fn open_bank_account() -> TestResult {
        let event_store = InMemoryEventStore::new();
        let repository = BankAccountRepository::new(event_store);
        let handler = OpenBankAccountHandler::new(repository);
        let cmd = OpenBankAccount::new(ACCOUNT_ID, CUSTOMER_ID);

        assert_eq!(Ok(()), handler.handle(cmd));
        Ok(())
    }

    #[test]
    fn deposit_money() -> TestResult {
        let event_store = InMemoryEventStore::new();
        let repository = BankAccountRepository::new(event_store);
        let mut handler = DepositHandler::new(repository);
        let cmd = Deposit::new(ACCOUNT_ID, CUSTOMER_ID);

        assert_eq!(Ok(()), handler.handle(cmd));
        Ok(())
    }

    #[test]
    fn withdraw_money() -> TestResult {
        let event_store = InMemoryEventStore::new();
        let repository = BankAccountRepository::new(event_store);
        let handler = WithdrawHandler::new(repository);
        let cmd = Withdraw::new(ACCOUNT_ID, CUSTOMER_ID);

        assert_eq!(Ok(()), handler.handle(cmd));
        Ok(())
    }

    #[test]
    fn withdrawing_money_refused() -> TestResult {
        let event_store = InMemoryEventStore::new();
        let repository = BankAccountRepository::new(event_store);
        let handler = WithdrawHandler::new(repository);
        let cmd = Withdraw::new(ACCOUNT_ID, CUSTOMER_ID);

        assert_eq!(Ok(()), handler.handle(cmd));
        Ok(())
    }
}
