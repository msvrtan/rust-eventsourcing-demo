use crate::model::{BankAccountId, CustomerId};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

//
//     Events
//
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum BankAccountEvent {
    BankAccountOpened(BankAccountOpened),
    Credited(BankAccountCredited),
    Debited(BankAccountDebited),
    WithdrawalRefused(BankAccountWithdrawalRefused),
}

impl BankAccountEvent {
    pub fn get_aggregate_id(&self) -> u64 {
        match &self {
            BankAccountEvent::BankAccountOpened(payload) => payload.id,
            BankAccountEvent::Credited(payload) => payload.id,
            BankAccountEvent::Debited(payload) => payload.id,
            BankAccountEvent::WithdrawalRefused(payload) => payload.id,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct BankAccountOpened {
    pub id: BankAccountId,
    pub customer_id: CustomerId,
    pub opened_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct BankAccountCredited {
    pub id: BankAccountId,
    pub amount: u64,
    pub credited_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct BankAccountDebited {
    pub id: BankAccountId,
    pub amount: u64,
    pub debited_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct BankAccountWithdrawalRefused {
    pub id: BankAccountId,
    pub amount: u64,
    pub balance: u64,
    pub refused_at: DateTime<Utc>,
}

impl BankAccountEvent {
    pub fn acc_opened(id: BankAccountId, customer_id: CustomerId) -> BankAccountEvent {
        BankAccountEvent::BankAccountOpened(BankAccountOpened {
            id: id,
            customer_id: customer_id,
            opened_at: Utc::now().round_subsecs(0),
        })
    }
    pub fn credited(id: BankAccountId, amount: u64) -> BankAccountEvent {
        BankAccountEvent::Credited(BankAccountCredited {
            id: id,
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
