//! State transition types
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use crate::{
    borsh_state::{BorshState, InitBorshState},
    error::Error,
};

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    program_pack::IsInitialized,
};

struct Decimal {
    pub value: u128,
    pub decimals: u32,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, Default, PartialEq)]
pub struct PublicKey(pub [u8; 32]);

impl PublicKey {
    pub fn is_account(&self, info: &AccountInfo) -> bool {
        self.eq(&PublicKey(info.key.to_bytes()))
    }
}

impl<'a> From<&'a AccountInfo<'a>> for PublicKey {
    fn from(info: &'a AccountInfo<'a>) -> Self {
        PublicKey(info.key.to_bytes())
    }
}

pub trait Authority {
    fn authority(&self) -> &PublicKey;

    fn authorize(&self, account: &AccountInfo) -> ProgramResult {
        if !account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if self.authority().0 != account.key.to_bytes() {
            return Err(Error::OwnerMismatch)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, Default, PartialEq)]
pub struct ImpermenantLossStopLossConfig {
    /// min_change_factor for stop-loss
    pub min_change_factor: u64,

    // token_a liquidity PK
    pub token_a: PublicKey,

    // token_b liquidity PK
    pub token_b: PublicKey,

    // token_a starting price
    pub token_a_starting_price: u64,

    // token_b starting price
    pub token_b_starting_price: u64,

    // liquidity_contract PK
    pub liquidity_contract: PublicKey
}

/// Aggregator data.
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, Default, PartialEq)]
pub struct ImpermenantLossStopLoss {
    pub config: ImpermenantLossStopLossConfig,
    /// is initialized
    pub is_initialized: bool,
    /// authority
    pub owner: PublicKey,
    /// min_change_factor for stop-loss
    pub min_change_factor: u64,

    // token_a liquidity PK
    pub token_a: PublicKey,

    // token_b liquidity PK
    pub token_b: PublicKey,

    // token_a starting price
    pub token_a_starting_price: u64,

    // token_b starting price
    pub token_b_starting_price: u64,

    // liquidity_contract PK
    pub liquidity_contract: PublicKey
}

impl ImpermenantLossStopLoss {
}

impl Authority for ImpermenantLossStopLoss {
    fn authority(&self) -> &PublicKey {
        &self.owner
    }
}
impl IsInitialized for ImpermenantLossStopLoss {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
impl BorshState for ImpermenantLossStopLoss {}
impl InitBorshState for ImpermenantLossStopLoss {}


mod tests {
    use crate::borsh_utils;

    use super::*;

    #[test]
    fn test_packed_len() {
        println!(
            "ImpermenantLossStopLoss len: {}",
            borsh_utils::get_packed_len::<ImpermenantLossStopLoss>()
        );

        println!(
            "ImpermenantLossStopLossConfig len: {}",
            borsh_utils::get_packed_len::<ImpermenantLossStopLossConfig>()
        );
    }
}
