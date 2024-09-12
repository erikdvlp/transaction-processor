use serde::Deserialize;

use super::id::{ClientID, TransactionID};

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    transaction_type: TransactionType,
    #[serde(rename = "client")]
    client_id: ClientID,
    #[serde(rename = "tx")]
    transaction_id: TransactionID,
    #[serde(rename = "amount")]
    amount: Option<f32>,
    #[serde(default)]
    in_dispute: bool,
}

impl Transaction {
    #[allow(unused)]
    pub fn new(
        transaction_type: TransactionType,
        client_id: ClientID,
        transaction_id: TransactionID,
        amount: Option<f32>,
    ) -> Self {
        Transaction {
            transaction_type,
            client_id,
            transaction_id,
            amount,
            in_dispute: false,
        }
    }

    pub fn transaction_type(&self) -> TransactionType {
        self.transaction_type
    }

    pub fn client_id(&self) -> ClientID {
        self.client_id
    }

    pub fn transaction_id(&self) -> TransactionID {
        self.transaction_id
    }

    pub fn amount(&self) -> Option<f32> {
        self.amount
    }

    pub fn in_dispute(&self) -> bool {
        self.in_dispute
    }

    pub fn set_dispute(&mut self, in_dispute: bool) {
        self.in_dispute = in_dispute;
    }
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
