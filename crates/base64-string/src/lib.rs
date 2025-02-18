use solders_message::VersionedMessage;
use solders_transaction::{Transaction, VersionedTransaction};
extern crate base64;
use serde::{Deserialize, Serialize};
use {
    solana_message::VersionedMessage as VersionedMessageOriginal,
    solana_transaction::{
        versioned::VersionedTransaction as VersionedTransactionOriginal,
        Transaction as TransactionOriginal,
    },
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Base64String(pub String);

impl From<Transaction> for Base64String {
    fn from(tx: Transaction) -> Self {
        Self(base64::encode(bincode::serialize(&tx).unwrap()))
    }
}

impl From<Base64String> for Transaction {
    fn from(tx: Base64String) -> Self {
        let bytes = base64::decode(tx.0).unwrap();
        bincode::deserialize::<TransactionOriginal>(&bytes)
            .unwrap()
            .into()
    }
}

impl From<VersionedTransaction> for Base64String {
    fn from(tx: VersionedTransaction) -> Self {
        Self(base64::encode(bincode::serialize(&tx).unwrap()))
    }
}

impl From<Base64String> for VersionedTransaction {
    fn from(tx: Base64String) -> Self {
        let bytes = base64::decode(tx.0).unwrap();
        bincode::deserialize::<VersionedTransactionOriginal>(&bytes)
            .unwrap()
            .into()
    }
}

impl From<Vec<u8>> for Base64String {
    fn from(tx: Vec<u8>) -> Self {
        Self(base64::encode(tx))
    }
}

impl From<Base64String> for Vec<u8> {
    fn from(tx: Base64String) -> Self {
        base64::decode(tx.0).unwrap()
    }
}

impl From<VersionedMessage> for Base64String {
    fn from(m: VersionedMessage) -> Self {
        let orig = VersionedMessageOriginal::from(m);
        Self(base64::encode(orig.serialize()))
    }
}

impl From<Base64String> for VersionedMessage {
    fn from(m: Base64String) -> Self {
        let bytes = base64::decode(m.0).unwrap();
        bincode::deserialize::<VersionedMessageOriginal>(&bytes)
            .unwrap()
            .into()
    }
}
