use super::io::AccountsMap;
use super::io::TransactionsMap;
use crate::models::account::Account;
use crate::models::transaction::Transaction;
use crate::models::transaction::TransactionType;

pub fn process_transaction(
    transaction: Transaction,
    accounts: &mut AccountsMap,
    transactions: &mut TransactionsMap,
) {
    // Get account from memory or create a new account
    accounts
        .entry(transaction.client_id())
        .or_insert(Account::new(transaction.client_id()));

    // Apply transaction to account and
    // save or update transaction in memory
    if let Some(account) = accounts.get_mut(&transaction.client_id()) {
        match transaction.transaction_type() {
            TransactionType::Deposit => {
                process_deposit_transcation(account, transaction, transactions)
            }
            TransactionType::Withdrawal => {
                process_withdraw_transaction(account, transaction, transactions)
            }
            TransactionType::Dispute => {
                process_dispute_transaction(account, transaction, transactions)
            }
            TransactionType::Resolve => {
                process_resolve_transaction(account, transaction, transactions)
            }
            TransactionType::Chargeback => {
                process_chargeback_transaction(account, transaction, transactions)
            }
        }
    }
}

/// Processes a deposit transaction.
/// Increases the corresponding account's available and total funds.
/// Adds the transaction to memory.
fn process_deposit_transcation(
    account: &mut Account,
    transaction: Transaction,
    transactions: &mut TransactionsMap,
) {
    if let Some(amount) = transaction.amount() {
        account.add(amount);
        transactions.insert(transaction.transaction_id(), transaction);
    }
}

/// Processes a withdraw transaction.
/// Decreases the corresponding account's available and total funds if the account has sufficient available funds.
/// Adds the transaction to memory.
fn process_withdraw_transaction(
    account: &mut Account,
    transaction: Transaction,
    transactions: &mut TransactionsMap,
) {
    if let Some(amount) = transaction.amount() {
        account.subtract(amount);
        transactions.insert(transaction.transaction_id(), transaction);
    }
}

/// Processes a dispute transaction.
/// Gets a previous transaction from memory and holds that transaction's funds in the corresponding account.
/// Updates the previous transaction in memory to mark it as in dispute.
fn process_dispute_transaction(
    account: &mut Account,
    transaction: Transaction,
    transactions: &mut TransactionsMap,
) {
    if let Some(prev_transaction) = transactions.get_mut(&transaction.transaction_id()) {
        if !prev_transaction.in_dispute() && transaction.client_id() == prev_transaction.client_id()
        {
            if let Some(amount) = prev_transaction.amount() {
                account.hold(amount);
                prev_transaction.set_dispute(true);
            }
        }
    }
}

/// Processes a resolve transaction.
/// Gets a previous transaction from memory and, if it is disputed, releases that transaction's funds in the corresponding account.
/// Updates the previous transaction in memory to mark it as not in dispute.
fn process_resolve_transaction(
    account: &mut Account,
    transaction: Transaction,
    transactions: &mut TransactionsMap,
) {
    if let Some(prev_transaction) = transactions.get_mut(&transaction.transaction_id()) {
        if prev_transaction.in_dispute() && transaction.client_id() == prev_transaction.client_id()
        {
            if let Some(amount) = prev_transaction.amount() {
                account.release(amount);
                prev_transaction.set_dispute(false);
            }
        }
    }
}

/// Processes a chargeback transaction.
/// Gets a previous transaction from memory and, if it is disputed, decreases the corresponding account's held and total funds and locks the corresponding account.
/// Updates the previous transaction in memory to mark it as not in dispute.
fn process_chargeback_transaction(
    account: &mut Account,
    transaction: Transaction,
    transactions: &mut TransactionsMap,
) {
    if let Some(prev_transaction) = transactions.get_mut(&transaction.transaction_id()) {
        if prev_transaction.in_dispute() && transaction.client_id() == prev_transaction.client_id()
        {
            if let Some(amount) = prev_transaction.amount() {
                account.chargeback(amount);
                prev_transaction.set_dispute(false);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn run_process_transaction(input: Vec<Transaction>) -> (AccountsMap, TransactionsMap) {
        let mut accounts: AccountsMap = HashMap::new();
        let mut transactions: TransactionsMap = HashMap::new();
        for transaction in input {
            process_transaction(transaction, &mut accounts, &mut transactions);
        }
        (accounts, transactions)
    }

    #[test]
    fn test_process_transaction_deposit_withdraw() {
        let input: Vec<Transaction> = vec![
            Transaction::new(TransactionType::Deposit, 1, 1, Some(1.0)),
            Transaction::new(TransactionType::Deposit, 2, 2, Some(2.0)),
            Transaction::new(TransactionType::Deposit, 1, 3, Some(2.0)),
            Transaction::new(TransactionType::Withdrawal, 1, 4, Some(1.5)),
            Transaction::new(TransactionType::Withdrawal, 2, 5, Some(3.0)),
        ];
        let (accounts, transactions) = run_process_transaction(input);

        assert_eq!(accounts.keys().len(), 2);
        assert_eq!(transactions.keys().len(), 5);

        let account_1 = accounts.get(&1).unwrap();
        assert_eq!(account_1.available(), 1.5);
        assert_eq!(account_1.held(), 0.0);
        assert_eq!(account_1.total(), 1.5);
        assert_eq!(account_1.locked(), false);

        let account_2 = accounts.get(&2).unwrap();
        assert_eq!(account_2.available(), 2.0);
        assert_eq!(account_2.held(), 0.0);
        assert_eq!(account_2.total(), 2.0);
        assert_eq!(account_2.locked(), false);
    }

    #[test]
    fn test_process_transaction_dispute_resolve_chargeback() {
        let input: Vec<Transaction> = vec![
            Transaction::new(TransactionType::Deposit, 1, 1, Some(5.0)),
            Transaction::new(TransactionType::Dispute, 1, 1, None),
            Transaction::new(TransactionType::Resolve, 1, 1, None),
            Transaction::new(TransactionType::Deposit, 2, 2, Some(5.0)),
            Transaction::new(TransactionType::Dispute, 2, 2, None),
            Transaction::new(TransactionType::Chargeback, 2, 2, None),
        ];
        let (accounts, transactions) = run_process_transaction(input);

        assert_eq!(accounts.keys().len(), 2);
        assert_eq!(transactions.keys().len(), 2);

        let account_1 = accounts.get(&1).unwrap();
        assert_eq!(account_1.available(), 5.0);
        assert_eq!(account_1.held(), 0.0);
        assert_eq!(account_1.total(), 5.0);
        assert_eq!(account_1.locked(), false);

        let account_2 = accounts.get(&2).unwrap();
        assert_eq!(account_2.available(), 0.0);
        assert_eq!(account_2.held(), 0.0);
        assert_eq!(account_2.total(), 0.0);
        assert_eq!(account_2.locked(), true);
    }
}
