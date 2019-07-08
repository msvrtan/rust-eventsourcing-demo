//
//     Types,models
//

use crate::error::BankAccountError;
use crate::event::{
    BankAccountCredited, BankAccountDebited, BankAccountEvent, BankAccountOpened, DomainMessage,
};

pub type BankAccountId = u64;
pub type CustomerId = u64;

pub trait Id {}

pub trait Event {}

impl Id for BankAccountId {}

type OkOrError = Result<(), BankAccountError>;

///
/// ===========================================================================
///
#[derive(Debug)]
pub struct BankAccountState {
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
            Err(BankAccountError::NoState)
        }
    }
    ///
    /// ===========================================================================
    ///
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
    ///
    /// ===========================================================================
    ///

    fn record_event(&mut self, event: &BankAccountEvent) -> OkOrError {
        self.apply_event(&event)?;

        let message = DomainMessage::new(self.get_id(), *event, self.generation);

        self.new_events.push(message);
        Ok(())
    }

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
        self.state = Some(BankAccountState::new(e.id, e.customer_id));
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

    ///
    /// ===========================================================================
    ///
    pub fn get_new_events(&self) -> &Vec<DomainMessage> {
        return &self.new_events;
    }
}

///
/// ===========================================================================
///
#[cfg(test)]
mod tests {
    use crate::event::{BankAccountEvent, DomainMessage};
    use crate::model::{BankAccountAggregate, BankAccountError};
    type TestResult = Result<(), BankAccountError>;

    #[test]
    fn open_bank_account() -> TestResult {
        // Arrange
        let id = 100;
        let customer_id = 4;

        let acc_opened = BankAccountEvent::acc_opened(id, customer_id);

        let expected = vec![DomainMessage::new(id, acc_opened, 1)];

        // Act
        let mut agg = BankAccountAggregate::new();
        agg.open_acc(id, customer_id)?;
        let events = agg.get_new_events();

        // Assert
        assert_eq!(expected, *events);
        Ok(())
    }

    #[test]
    fn deposit_money() -> TestResult {
        // Arrange
        let id = 100;
        let customer_id = 4;

        let acc_opened = BankAccountEvent::acc_opened(id, customer_id);
        let acc_credited = BankAccountEvent::credited(67);

        let expected = vec![
            DomainMessage::new(id, acc_opened, 1),
            DomainMessage::new(id, acc_credited, 2),
        ];

        let mut agg = BankAccountAggregate::new();
        agg.open_acc(id, customer_id)?;
        // Act
        agg.deposit(67)?;
        let events = agg.get_new_events();

        // Assert
        assert_eq!(expected, *events);
        Ok(())
    }

    #[test]
    fn withdraw_money() -> TestResult {
        // Arrange
        let id = 100;
        let customer_id = 4;

        let acc_opened = BankAccountEvent::acc_opened(id, customer_id);
        let acc_credited = BankAccountEvent::credited(67);
        let acc_debited = BankAccountEvent::debited(id, 34);

        let expected = vec![
            DomainMessage::new(id, acc_opened, 1),
            DomainMessage::new(id, acc_credited, 2),
            DomainMessage::new(id, acc_debited, 3),
        ];

        let mut agg = BankAccountAggregate::new();
        agg.open_acc(id, customer_id)?;
        agg.deposit(67)?;
        // Act
        agg.withdraw(34)?;
        let events = agg.get_new_events();

        // Assert
        assert_eq!(expected, *events);
        Ok(())
    }

    #[test]
    fn withdrawing_money_refused() -> TestResult {
        // Arrange
        let (id, customer_id) = (100, 4);

        let acc_opened = BankAccountEvent::acc_opened(id, customer_id);
        let acc_credited = BankAccountEvent::credited(67);
        let refused = BankAccountEvent::withdrawal_refused(id, 100, 67);

        let expected = vec![
            DomainMessage::new(id, acc_opened, 1),
            DomainMessage::new(id, acc_credited, 2),
            DomainMessage::new(id, refused, 3),
        ];

        let mut agg = BankAccountAggregate::new();
        agg.open_acc(id, customer_id)?;
        agg.deposit(67)?;

        // Act
        agg.withdraw(100)?;
        let events = agg.get_new_events();

        // Assert
        assert_eq!(expected, *events);
        Ok(())
    }

    #[test]
    fn withdrawing_money_refused2() -> TestResult {
        // Arrange
        let id = 100;
        let customer_id = 4;

        let acc_opened = BankAccountEvent::acc_opened(id, customer_id);
        let acc_credited = BankAccountEvent::credited(67);
        let refused = BankAccountEvent::withdrawal_refused(id, 100, 67);

        let initial_events = vec![acc_opened, acc_credited];

        let mut agg = build_aggregate_with(initial_events);

        let expected = vec![DomainMessage::new(id, refused, 3)];

        // Act
        agg.withdraw(100)?;
        let events = agg.get_new_events();

        // Assert
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
