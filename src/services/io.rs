use super::transaction_processor::process_transaction;
use crate::models::account::Account;
use crate::models::checkpoint::Checkpoint;
use crate::models::id::ClientID;
use crate::models::id::TransactionID;
use crate::models::transaction::Transaction;
use csv::ReaderBuilder;
use log::error;
use log::info;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Write;
use std::fs;
use std::fs::File;
use std::io::Write as IOWrite;

pub type AccountsMap = HashMap<ClientID, Account>;
pub type TransactionsMap = HashMap<TransactionID, Transaction>;
const CHECKPOINT_PATH: &str = "temp/checkpoint.json";

/// Reads transactions from given input CSV file.
/// Processes each parsed transaction one by one.
/// When a block is completed, writes a checkpoint to disk.
pub fn read_transactions(
    input_file: &str,
    block_size: u32,
) -> Result<Vec<Account>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_file)?;

    let mut accounts: AccountsMap = HashMap::new();
    let mut transactions: TransactionsMap = HashMap::new();
    let mut line = 0;
    for result in reader.deserialize::<Transaction>() {
        match result {
            Ok(transaction) => process_transaction(transaction, &mut accounts, &mut transactions),
            Err(e) => error!("Failed to parse transaction on line {}: {}", line, e),
        }
        line += 1;
        if line % block_size == 0 {
            match write_checkpoint(line, &accounts, &transactions) {
                Ok(_) => info!("Wrote current state checkpoint to disk at line {}", line),
                Err(e) => error!(
                    "Failed to write current state checkpoint to disk at line {}: {}",
                    line, e
                ),
            }
        }
    }

    let _ = clear_checkpoint();
    Ok(accounts.into_values().collect())
}

/// Writes the current state of accounts and transactions to disk.
fn write_checkpoint(
    line: u32,
    accounts: &AccountsMap,
    transactions: &TransactionsMap,
) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("temp")?;

    let checkpoint = Checkpoint {
        line,
        accounts: accounts.clone(),
        transactions: transactions.clone(),
    };
    let checkpoint_json = serde_json::to_string(&checkpoint)?;
    let mut checkpoint_file = File::create(CHECKPOINT_PATH)?;
    checkpoint_file.write_all(checkpoint_json.as_bytes())?;

    Ok(())
}

/// Deletes the current state of accounts and transactions from disk.
fn clear_checkpoint() -> Result<(), Box<dyn Error>> {
    fs::remove_file(CHECKPOINT_PATH)?;

    Ok(())
}

/// Writes account data to standard output.
pub fn write_accounts(accounts: Vec<Account>) {
    println!("client,available,held,total,locked");
    let mut output = String::new();
    for account in accounts {
        writeln!(
            output,
            "{},{:.4},{:.4},{:.4},{}",
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
