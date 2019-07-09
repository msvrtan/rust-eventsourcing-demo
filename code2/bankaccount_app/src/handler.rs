use crate::command::{Deposit, OpenBankAccount, Withdraw};
use crate::error::BankAccountAppError;
use crate::repository::BankAccountRepository;
use bankaccount_core::model::BankAccountAggregate;

struct OpenBankAccountHandler {
    repository: Box<dyn BankAccountRepository>,
}

impl OpenBankAccountHandler {
    fn handle(self, command: OpenBankAccount) -> Result<(), BankAccountAppError> {
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
    repository: Box<BankAccountRepository>,
}

impl DepositHandler {
    fn handle(self, command: Deposit) -> Result<(), ()> {
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
    repository: Box<BankAccountRepository>,
}

impl WithdrawHandler {
    fn handle(self, command: Withdraw) -> Result<(), ()> {
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
    use crate::command::OpenBankAccount;
    use crate::handler::OpenBankAccountHandler;
    use crate::repository::InMemoryBankAccountRepository;
    use bankaccount_core::{BankAccountId, CustomerId};

    type TestResult = Result<(), ()>;

    const ACCOUNT_ID: BankAccountId = 123;
    const CUSTOMER_ID: CustomerId = 5000;

    #[test]
    fn open_bank_account() -> TestResult {
        let repository = InMemoryBankAccountRepository {};
        let handler = OpenBankAccountHandler {
            repository: Box::new(repository),
        };
        let cmd = OpenBankAccount::new(ACCOUNT_ID, CUSTOMER_ID);

        handler.handle(cmd)
    }

    #[test]
    fn deposit_money() -> TestResult {
        Ok(())
    }

    #[test]
    fn withdraw_money() -> TestResult {
        Ok(())
    }

    #[test]
    fn withdrawing_money_refused() -> TestResult {
        Ok(())
    }

}
