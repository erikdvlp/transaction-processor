use serde::Serialize;

use crate::services::io::AccountsMap;
use crate::services::io::TransactionsMap;

#[derive(Serialize)]
pub struct Checkpoint {
    pub line: u32,
    pub accounts: AccountsMap,
    pub transactions: TransactionsMap,
}
