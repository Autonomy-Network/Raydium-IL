#![forbid(unsafe_code)]

pub mod borsh_state;
pub mod borsh_utils;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

use crate::error::Error;
use borsh_state::InitBorshState;
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized
};

// expected struct `solana_program::pubkey::Pubkey`, found struct `state::PublicKey`rustc(E0308)
use crate::state::PublicKey;

use state::{ImpermenantLossStopLoss};

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

/// Read resolved min_change_factor from the ImpermenantLossStopLoss
pub fn read_min_change_factor(
    impermenant_loss_stop_loss_info: &AccountInfo,
) -> Result<u64, ProgramError> {
    let impermenant_loss_stop_loss = ImpermenantLossStopLoss::load_initialized(&impermenant_loss_stop_loss_info)?;

    if !impermenant_loss_stop_loss.is_initialized() {
        return Err(Error::OwnerMismatch)?;
    }

    Ok(impermenant_loss_stop_loss.min_change_factor)
}

/// Read resolved min_change_factor from the ImpermenantLossStopLoss
pub fn read_token_a(
    impermenant_loss_stop_loss_info: &AccountInfo,
) -> Result<PublicKey, ProgramError> {
    let impermenant_loss_stop_loss = ImpermenantLossStopLoss::load_initialized(&impermenant_loss_stop_loss_info)?;

    if !impermenant_loss_stop_loss.is_initialized() {
        return Err(Error::UnknownError)?;
    }

    Ok(impermenant_loss_stop_loss.token_a)
}
// Export current sdk types for downstream users building with a different
pub use solana_program;
