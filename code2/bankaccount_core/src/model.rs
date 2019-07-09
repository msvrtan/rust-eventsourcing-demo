//
//     Types,models
//

use crate::error::BankAccountError;
use crate::event::*;
use crate::{BankAccountId, CustomerId};

type OkOrError = Result<(), BankAccountError>;

///
/// ===========================================================================
///

#[derive(Debug)]
struct BankAccountState {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
    pub balance: u64,
}

impl BankAccountState {
    pub fn new(id: BankAccountId, customer_id: CustomerId) -> BankAccountState {
        BankAccountState {
            id: id,
            customer_id: customer_id,
            balance: 0,
        }
    }
}

///
/// ===========================================================================
///

#[derive(Debug)]
pub struct BankAccountAggregate {
    state: Option<BankAccountState>,
    generation: u8,
    new_events: Vec<DomainMessage>,
}

impl BankAccountAggregate {
    pub fn new() -> Self {
        BankAccountAggregate {
            state: None,
            generation: 0,
            new_events: Vec::new(),
        }
    }

    pub fn get_id(&mut self) -> BankAccountId {
        self.state.as_ref().unwrap().id
    }

    fn get_state(&mut self) -> Result<&mut BankAccountState, BankAccountError> {
        if let Some(state) = &mut self.state {
            Ok(state)
        } else {
            Err(BankAccountError::NotOpened)
        }
    }
    pub fn get_new_events(&self) -> &Vec<DomainMessage> {
        return &self.new_events;
    }
    fn record_event(&mut self, event: &BankAccountEvent) -> OkOrError {
        self.apply_event(&event)?;

        let message = DomainMessage::new(self.get_id(), *event, self.generation);

        self.new_events.push(message);
        Ok(())
    }
}

impl BankAccountAggregate {
    pub fn open_acc(&mut self, id: BankAccountId, customer_id: CustomerId) -> OkOrError {
        let event = BankAccountEvent::acc_opened(id, customer_id);

        self.record_event(&event)
    }
    pub fn deposit(&mut self, amount: u64) -> OkOrError {
        let event = BankAccountEvent::credited(amount);

        self.record_event(&event)
    }
    pub fn withdraw(&mut self, amount: u64) -> OkOrError {
        let event = match &self.state {
            Some(state) => match state.balance >= amount {
                true => BankAccountEvent::debited(self.state.as_ref().unwrap().id, amount),
                false => BankAccountEvent::withdrawal_refused(
                    self.state.as_ref().unwrap().id,
                    amount,
                    state.balance,
                ),
            },
            _ => panic!("TODO"),
        };

        self.record_event(&event)
    }
}

impl BankAccountAggregate {
    fn apply_event(&mut self, event: &BankAccountEvent) -> OkOrError {
        self.generation += 1;
        match event {
            BankAccountEvent::Opened(payload) => self.account_opened(payload),
            BankAccountEvent::Credited(payload) => self.account_credited(payload),
            BankAccountEvent::Debited(payload) => self.account_debited(payload),
            BankAccountEvent::WithdrawalRefused(_payload) => Ok(()),
        }
    }

    fn account_opened(&mut self, e: &BankAccountOpened) -> OkOrError {
        if let Some(_) = self.state {
            return Err(BankAccountError::AlreadyOpened);
        } else {
            self.state = Some(BankAccountState::new(e.id, e.customer_id));
        }

        Ok(())
    }

    fn account_credited(&mut self, e: &BankAccountCredited) -> OkOrError {
        let state = self.get_state()?;
        state.balance += e.amount;
        Ok(())
    }

    fn account_debited(&mut self, e: &BankAccountDebited) -> OkOrError {
        let state = self.get_state()?;
        state.balance -= e.amount;
        Ok(())
    }
}

///
/// ===========================================================================
///
#[cfg(test)]
mod tests {
    use crate::event::{BankAccountEvent, DomainMessage};
    use crate::model::{BankAccountAggregate, BankAccountError};
    use crate::{BankAccountId, CustomerId};

    type TestResult = Result<(), BankAccountError>;

    const ACCOUNT_ID: BankAccountId = 123;
    const CUSTOMER_ID: CustomerId = 5000;

    #[test]
    fn open_bank_account() -> TestResult {
        // Arrange
        let acc_opened = BankAccountEvent::acc_opened(ACCOUNT_ID, CUSTOMER_ID);

        let mut agg = BankAccountAggregate::new();

        let expected = vec![DomainMessage::new(ACCOUNT_ID, acc_opened, 1)];

        // Act
        agg.open_acc(ACCOUNT_ID, CUSTOMER_ID)?;

        // Assert
        let events = agg.get_new_events();
        assert_eq!(expected, *events);
        Ok(())
    }

    #[test]
    fn deposit_money() -> TestResult {
        // Arrange
        let acc_opened = BankAccountEvent::acc_opened(ACCOUNT_ID, CUSTOMER_ID);
        let acc_credited = BankAccountEvent::credited(67);

        let initial_events = vec![acc_opened];
        let mut agg = build_aggregate_with(initial_events);

        let expected = vec![DomainMessage::new(ACCOUNT_ID, acc_credited, 2)];

        // Act
        agg.deposit(67)?;

        // Assert
        let events = agg.get_new_events();
        assert_eq!(expected, *events);
        Ok(())
    }

    #[test]
    fn withdraw_money() -> TestResult {
        // Arrange
        let acc_opened = BankAccountEvent::acc_opened(ACCOUNT_ID, CUSTOMER_ID);
        let acc_credited = BankAccountEvent::credited(67);
        let acc_debited = BankAccountEvent::debited(ACCOUNT_ID, 34);

        let initial_events = vec![acc_opened, acc_credited];
        let mut agg = build_aggregate_with(initial_events);

        let expected = vec![DomainMessage::new(ACCOUNT_ID, acc_debited, 3)];

        // Act
        agg.withdraw(34)?;

        // Assert
        let events = agg.get_new_events();
        assert_eq!(expected, *events);
        Ok(())
    }

    #[test]
    fn withdrawing_money_refused() -> TestResult {
        // Arrange
        let acc_opened = BankAccountEvent::acc_opened(ACCOUNT_ID, CUSTOMER_ID);
        let acc_credited = BankAccountEvent::credited(67);
        let refused = BankAccountEvent::withdrawal_refused(ACCOUNT_ID, 100, 67);

        let initial_events = vec![acc_opened, acc_credited];
        let mut agg = build_aggregate_with(initial_events);

        let expected = vec![DomainMessage::new(ACCOUNT_ID, refused, 3)];

        // Act
        agg.withdraw(100)?;

        // Assert
        let events = agg.get_new_events();

        assert_eq!(expected, *events);
        Ok(())
    }

    fn build_aggregate_with(events: Vec<BankAccountEvent>) -> BankAccountAggregate {
        let mut agg = BankAccountAggregate::new();
        for event in events {
            agg.apply_event(&event).unwrap();
        }

        return agg;
    }

}
