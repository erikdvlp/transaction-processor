use super::transaction_processor::process_transaction;
use crate::models::account::Account;
use crate::models::id::{ClientID, TransactionID};
use crate::models::transaction::Transaction;
use csv::ReaderBuilder;
use log::error;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;

pub type AccountsMap = HashMap<ClientID, Account>;
pub type TransactionsMap = HashMap<TransactionID, Transaction>;

pub fn read_transactions(input_file: &str) -> Result<Vec<Account>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_file)?;

    let mut accounts: AccountsMap = HashMap::new();
    let mut transactions: TransactionsMap = HashMap::new();
    for result in reader.deserialize::<Transaction>() {
        match result {
            Ok(transaction) => process_transaction(transaction, &mut accounts, &mut transactions),
            Err(e) => error!("Failed to parse transaction: {}", e),
        }
    }

    Ok(accounts.into_values().collect())
}

pub fn write_accounts(accounts: Vec<Account>) {
    println!("client,available,held,total,locked");
    let mut output = String::new();
    for account in accounts {
        writeln!(
            output,
            "{},{},{},{},{}",
            account.client_id(),
            account.available(),
            account.held(),
            account.total(),
            account.locked()
        )
        .unwrap();
    }
    println!("{}", output);
}
