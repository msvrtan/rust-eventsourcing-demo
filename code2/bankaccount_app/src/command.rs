use bankaccount_core::{BankAccountId, CustomerId};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct OpenBankAccount {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
}

impl OpenBankAccount {
    pub fn new(id: BankAccountId, customer_id: CustomerId) -> Self {
        OpenBankAccount { id, customer_id }
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Deposit {
    id: BankAccountId,
    pub amount: u64,
}

impl Deposit {
    pub fn new(id: BankAccountId, amount: u64) -> Self {
        Deposit { id, amount }
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Withdraw {
    id: BankAccountId,
    pub amount: u64,
}

impl Withdraw {
    pub fn new(id: BankAccountId, amount: u64) -> Self {
        Withdraw { id, amount }
    }
}
