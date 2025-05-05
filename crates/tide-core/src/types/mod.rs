pub mod account;

use serde::{Serialize, Deserialize};
use solana_sdk::{pubkey::Pubkey, signature::Signature, transaction::VersionedTransaction};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C, align(64))] // Cache line alignment
pub enum TideData {
    Transaction(TransactionData),
    Account(account::AccountData),
    Block(BlockData),
    Slot(SlotData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct TransactionData {
    pub signature: Signature,
    pub transaction: VersionedTransaction,
    pub slot: u64,
    pub timestamp: i64,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockData {
    pub slot: u64,
    pub parent_slot: u64,
    pub height: u64,
    pub timestamp: i64,
    pub blockhash: String,
    pub transactions_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotData {
    pub slot: u64,
    pub parent: Option<u64>,
    pub status: SlotStatus,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SlotStatus {
    Processed,
    Confirmed,
    Finalized,
}