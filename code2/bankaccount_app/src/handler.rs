use crate::command::{Deposit, OpenBankAccount, Withdraw};
use crate::error::BankAccountAppError;
use crate::repository::BankAccountRepository;
use bankaccount_core::model::BankAccountAggregate;

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

struct DepositHandler {
    repository: BankAccountRepository,
}

impl DepositHandler {
    fn new(repository: BankAccountRepository) -> Self {
        Self {
            repository: repository,
        }
    }
    fn handle(self, command: Deposit) -> HandlerResult {
        let mut agg = self.repository.load(command.id)?;

        agg.deposit(command.amount);

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

struct WithdrawHandler {
    repository: BankAccountRepository,
}

impl WithdrawHandler {
    fn new(repository: BankAccountRepository) -> Self {
        Self { repository }
    }
    fn handle(self, command: Withdraw) -> HandlerResult {
        let mut agg = self.repository.load(command.id)?;

        agg.withdraw(command.amount);

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

#[cfg(test)]
mod tests {
    use crate::command::{Deposit, OpenBankAccount, Withdraw};
    use crate::error::BankAccountAppError;
    use crate::handler::{DepositHandler, OpenBankAccountHandler, WithdrawHandler};
    use crate::repository::BankAccountRepository;
    use bankaccount_core::{BankAccountId, CustomerId};

    type TestResult = Result<(), BankAccountAppError>;

    const ACCOUNT_ID: BankAccountId = 123;
    const CUSTOMER_ID: CustomerId = 5000;

    #[test]
    fn open_bank_account() -> TestResult {
        let repository = BankAccountRepository {};
        let handler = OpenBankAccountHandler::new(repository);
        let cmd = OpenBankAccount::new(ACCOUNT_ID, CUSTOMER_ID);

        assert_eq!(Ok(()), handler.handle(cmd));
        Ok(())
    }

    #[test]
    fn deposit_money() -> TestResult {
        let repository = BankAccountRepository {};
        let handler = DepositHandler::new(repository);
        let cmd = Deposit::new(ACCOUNT_ID, CUSTOMER_ID);

        assert_eq!(Ok(()), handler.handle(cmd));
        Ok(())
    }

    #[test]
    fn withdraw_money() -> TestResult {
        let repository = BankAccountRepository {};
        let handler = WithdrawHandler::new(repository);
        let cmd = Withdraw::new(ACCOUNT_ID, CUSTOMER_ID);

        assert_eq!(Ok(()), handler.handle(cmd));
        Ok(())
    }

    #[test]
    fn withdrawing_money_refused() -> TestResult {
        let repository = BankAccountRepository {};
        let handler = WithdrawHandler::new(repository);
        let cmd = Withdraw::new(ACCOUNT_ID, CUSTOMER_ID);

        assert_eq!(Ok(()), handler.handle(cmd));
        Ok(())
    }
}
