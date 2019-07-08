use crate::model::{BankAccountId, CustomerId, Event};
use chrono::prelude::*;

//
//     Events
//
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BankAccountEvent {
    Opened(BankAccountOpened),
    Credited(BankAccountCredited),
    Debited(BankAccountDebited),
    WithdrawalRefused(BankAccountWithdrawalRefused),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BankAccountOpened {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
    pub opened_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BankAccountCredited {
    pub amount: u64,
    pub credited_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BankAccountDebited {
    pub id: BankAccountId,
    pub amount: u64,
    pub debited_at: DateTime<Utc>,
}
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BankAccountWithdrawalRefused {
    pub id: BankAccountId,
    pub amount: u64,
    pub balance: u64,
    pub refused_at: DateTime<Utc>,
}

impl BankAccountEvent {
    pub fn acc_opened(id: BankAccountId, customer_id: CustomerId) -> BankAccountEvent {
        BankAccountEvent::Opened(BankAccountOpened {
            id: id,
            customer_id: customer_id,
            opened_at: Utc::now().round_subsecs(0),
        })
    }
    pub fn credited(amount: u64) -> BankAccountEvent {
        BankAccountEvent::Credited(BankAccountCredited {
            amount: amount,
            credited_at: Utc::now().round_subsecs(0),
        })
    }
    pub fn debited(id: BankAccountId, amount: u64) -> BankAccountEvent {
        BankAccountEvent::Debited(BankAccountDebited {
            id: id,
            amount: amount,
            debited_at: Utc::now().round_subsecs(0),
        })
    }
    pub fn withdrawal_refused(id: BankAccountId, amount: u64, balance: u64) -> BankAccountEvent {
        BankAccountEvent::WithdrawalRefused(BankAccountWithdrawalRefused {
            id: id,
            amount: amount,
            balance: balance,
            refused_at: Utc::now().round_subsecs(0),
        })
    }
}

impl Event for BankAccountEvent {}

#[derive(Debug, PartialEq, Clone)]
pub struct DomainMessage {
    pub id: BankAccountId,
    pub event: BankAccountEvent,
    pub generation: u8,
    pub created_at: DateTime<Utc>,
}

impl DomainMessage {
    pub fn new(id: BankAccountId, event: BankAccountEvent, generation: u8) -> Self {
        DomainMessage {
            id: id,
            event: event,
            generation: generation,
            created_at: Utc::now().round_subsecs(0),
        }
    }
}
