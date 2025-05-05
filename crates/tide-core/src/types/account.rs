use serde::{Serialize, Deserialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C, align(64))]
pub struct AccountData {
    pub pubkey: Pubkey,
    pub owner: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub executable: bool,
    pub rent_epoch: u64,
    pub slot: u64,
    pub write_version: u64,
}

#[repr(C, align(64))]
pub struct SIMDAccount {
    pub pubkey: [u8; 32],
    pub owner: [u8; 32],
    pub lamports: u64,
    pub data_len: u32,
    pub data_ptr: *const u8,
    pub flags: u8, // bits: executable, etc.
    pub padding: [u8; 23], // Align to 64 bytes
}

impl AccountData {
    pub fn to_simd(&self) -> SIMDAccount {
        let flags = if self.executable { 0b1 } else { 0 };
        
        SIMDAccount {
            pubkey: self.pubkey.to_bytes(),
            owner: self.owner.to_bytes(),
            lamports: self.lamports,
            data_len: self.data.len() as u32,
            data_ptr: self.data.as_ptr(),
            flags,
            padding: [0; 23],
        }
    }
}