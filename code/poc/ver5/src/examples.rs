use crate::command::DepositMoney;
use crate::command::OpenBankAccount;
use crate::command::WithdrawMoney;
use crate::deposit::DepositHandler;
use crate::event::BankAccountEvent;
use crate::event_store::BankAccountEventStore;
use crate::open_bank_account::OpenBankAccountHandler;
use crate::prelude::JsonFileBankAccountEventStore;
use crate::repository::BankAccountRepository;
use crate::withdraw::WithdrawHandler;
use std::sync::Arc;

pub fn examples() {
    example_open_bank_account();
    example_deposit_money();
    example_withdraw_money();
    example_withdraw_refused();

    println!("Done!");
}

fn example_open_bank_account() {
    let (repo, event_store) = build_repo(Vec::new(),"open".to_string());
    let handler = OpenBankAccountHandler::new(repo);

    let result = handler.handle(OpenBankAccount::new(100, 20));

    println!("{:?}", &event_store.get_events(100));

    match result {
        Ok(()) => println!("Bank account opened"),
        _ => panic!("Opening bank account failed"),
    }
}

fn example_deposit_money() {
    let initial_events = vec![BankAccountEvent::acc_opened(201, 20)];

    let (repo, event_store) = build_repo(initial_events,"deposit".to_string());

    let handler = DepositHandler::new(repo);

    let result = handler.handle(DepositMoney::new(201, 10));

    println!("{:?}", &event_store.get_events(201));

    match result {
        Ok(()) => println!("Money deposited"),
        _ => panic!("Depositing failed"),
    }
}

fn example_withdraw_money() {
    let initial_events = vec![
        BankAccountEvent::acc_opened(302, 20),
        BankAccountEvent::credited(302, 49),
    ];
    let (repo, event_store) = build_repo(initial_events,"withdraw".to_string());

    let handler = WithdrawHandler::new(repo);

    let result = handler.handle(WithdrawMoney::new(302, 40));

    println!("{:?}", &event_store.get_events(302));

    match result {
        Ok(()) => println!("Money withdrawn"),
        _ => panic!("Withdrawing failed"),
    }
}

fn example_withdraw_refused() {
    let initial_events = vec![
        BankAccountEvent::acc_opened(403, 20),
        BankAccountEvent::credited(403, 49),
    ];
    let (repo, event_store) = build_repo(initial_events,"withdraw_refused".to_string());

    let handler = WithdrawHandler::new(repo);

    let result = handler.handle(WithdrawMoney::new(403, 50));

    println!("{:?}", &event_store.get_events(403));

    match result {
        Ok(()) => println!("Money withdrawal refused"),
        _ => panic!("Withdrawal refusing failed"),
    }
}

type BuildRepoResult = (
    Arc<BankAccountRepository>,
    Arc<JsonFileBankAccountEventStore>,
);

fn build_repo(initial_events: Vec<BankAccountEvent>, file_name: String) -> BuildRepoResult {

    let path ="/work/noob/rust-eventsourcing-demo/dev/code/poc/ver5/tmp/".to_string();
    let file_path = format!("{}{}.json", path, file_name);

    let event_store = Arc::new(JsonFileBankAccountEventStore::new(file_path));
    let event_store2 = event_store.clone();
    let repo = Arc::new(BankAccountRepository {
        event_store: event_store,
    });

    match event_store2.save_events(initial_events) {
        Ok(()) => println!("Initial events added"),
        _ => panic!("Setting up initial events failed"),
    }

    (repo, event_store2)
}
