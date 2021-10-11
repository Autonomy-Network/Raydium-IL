//! Instruction types
#![allow(dead_code)]

use crate::state::ImpermenantLossStopLossConfig;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Instructions supported by the program
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub enum Instruction {
    Initialize {
        config: ImpermenantLossStopLossConfig,
    },

    Configure {
        config: ImpermenantLossStopLossConfig,
    },

    OwnerAddLiquidity {
        amount_a: u64,
        amount_b: u64,
    },

    OwnerRemoveLiquidity {
        amount_a: u64,
        amount_b: u64,
    },

    AnyoneRemoveLiquidity {
        amount_a: u64,
        amount_b: u64,
    }
}
