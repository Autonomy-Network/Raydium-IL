//! Program state processor

use crate::{
    error::Error,
    instruction::{Instruction},
    state::{ImpermenantLossStopLoss, ImpermenantLossStopLossConfig, Authority},
};

// use spl_token::state;
use solana_program::{
    account_info::AccountInfo,
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::borsh_state::{BorshState, InitBorshState};

use borsh::BorshDeserialize;

struct Accounts<'a>(&'a [AccountInfo<'a>]);

impl<'a> Accounts<'a> {
    fn get(&self, i: usize) -> Result<&'a AccountInfo<'a>, ProgramError> {
        // fn get(&self, i: usize) -> Result<&AccountInfo, ProgramError> {
        // &accounts[input.token.account as usize]
        self.0.get(i).ok_or(ProgramError::NotEnoughAccountKeys)
    }

    fn get_rent(&self, i: usize) -> Result<Rent, ProgramError> {
        Rent::from_account_info(self.get(i)?)
    }

    fn get_clock(&self, i: usize) -> Result<Clock, ProgramError> {
        Clock::from_account_info(self.get(i)?)
    }
}

struct InitializeContext<'a> {
    rent: Rent,
    impermenant_loss_stop_loss: &'a AccountInfo<'a>,
    impermenant_loss_stop_loss_owner: &'a AccountInfo<'a>,   // signed
    config: ImpermenantLossStopLossConfig,
}

impl<'a> InitializeContext<'a> {
    fn process(&self) -> ProgramResult {
        if !self.impermenant_loss_stop_loss_owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut impermenant_loss_stop_loss_owner = ImpermenantLossStopLoss::init_uninitialized(self.impermenant_loss_stop_loss_owner)?;
        impermenant_loss_stop_loss_owner.is_initialized = true;
        impermenant_loss_stop_loss_owner.config = self.config.clone();
        impermenant_loss_stop_loss_owner.owner = self.impermenant_loss_stop_loss_owner.into();

        impermenant_loss_stop_loss_owner.save_exempt(self.impermenant_loss_stop_loss_owner, &self.rent)?;

        Ok(())
    }
}

struct ConfigureContext<'a> {
    impermenant_loss_stop_loss: &'a AccountInfo<'a>,
    impermenant_loss_stop_loss_owner: &'a AccountInfo<'a>,
    config: ImpermenantLossStopLossConfig,
}

impl<'a> ConfigureContext<'a> {
    fn process(&self) -> ProgramResult {
        let mut impermenant_loss_stop_loss = ImpermenantLossStopLoss::load_initialized(&self.impermenant_loss_stop_loss)?;
        impermenant_loss_stop_loss.authorize(self.impermenant_loss_stop_loss_owner)?;
        impermenant_loss_stop_loss.config = self.config.clone();
        impermenant_loss_stop_loss.save(self.impermenant_loss_stop_loss)?;

        Ok(())
    }
}

struct OwnerAddLiquidityContext<'a> {
    rent: Rent,
    impermenant_loss_stop_loss_token_a: &'a AccountInfo<'a>,
    impermenant_loss_stop_loss_token_b: &'a AccountInfo<'a>,
    impermenant_loss_stop_loss: &'a AccountInfo<'a>,
    impermenant_loss_stop_loss_owner: &'a AccountInfo<'a>, // signed
    amount_a: u64,
    amount_b: u64,
}

impl<'a> OwnerAddLiquidityContext<'a> {
    fn process(&self) -> ProgramResult {
        let impermenant_loss_stop_loss = ImpermenantLossStopLoss::load_initialized(self.impermenant_loss_stop_loss)?;
        msg!("loaded impermenant_loss_stop_loss");
        // authorization check for only the owner
        impermenant_loss_stop_loss.authorize(self.impermenant_loss_stop_loss_owner)?;

        msg!("auth check passed");
        // The SPL Token ensures that faucet and receiver are the same type of token
        let token_a_transfer = spl_token::instruction::transfer(
            &self.impermenant_loss_stop_loss_token_a.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &self.impermenant_loss_stop_loss.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &[],
            self.amount_a,
        )?;

        // The SPL Token ensures that faucet and receiver are the same type of token
        let token_b_transfer = spl_token::instruction::transfer(
            &self.impermenant_loss_stop_loss_token_b.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &self.impermenant_loss_stop_loss.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &[],
            self.amount_b,
        )?;

        // The SPL Token ensures that faucet and receiver are the same type of token
        let add_liquidity = spl_token::instruction::transfer(
            &self.impermenant_loss_stop_loss_token_b.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &self.impermenant_loss_stop_loss.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &[],
            self.amount_b,
        )?;

        // program_id: &Pubkey,
        // amm_id: &Pubkey,
        // amm_authority: &Pubkey,
        // amm_open_orders: &Pubkey,
        // amm_target_orders: &Pubkey,
        // lp_mint_address: &Pubkey,
        // pool_coin_token_account: &Pubkey,
        // pool_pc_token_account: &Pubkey,
        // serum_market: &Pubkey,
        // user_coin_token_account: &Pubkey,
        // user_pc_token_account: &Pubkey,
        // user_lp_token_account: &Pubkey,
        // user_owner: &Pubkey,
    
        // max_coin_amount: u64,
        // max_pc_amount: u64,
        // base_side: u64,

        // invoke_signed(
        //     &token_a_transfer,
        //     &[
        //         self.impermenant_loss_stop_loss_token_b.clone(),
        //         self.impermenant_loss_stop_loss.clone(),
        //         self.impermenant_loss_stop_loss_owner.clone(),
        //     ],
        //     &[&[self.faucet_owner_seed]],
        // )?;


        // call addLiquidity

        Ok(())
    }
}

struct OwnerRemoveLiquidityContext<'a> {
    rent: Rent,
    impermenant_loss_stop_loss_token_a: &'a AccountInfo<'a>,
    impermenant_loss_stop_loss_token_b: &'a AccountInfo<'a>,
    impermenant_loss_stop_loss: &'a AccountInfo<'a>,
    impermenant_loss_stop_loss_owner: &'a AccountInfo<'a>, // signed
    amount_a: u64,
    amount_b: u64,
}

impl<'a> OwnerRemoveLiquidityContext<'a> {
    fn process(&self) -> ProgramResult {
        let impermenant_loss_stop_loss = ImpermenantLossStopLoss::load_initialized(self.impermenant_loss_stop_loss)?;
        msg!("loaded impermenant_loss_stop_loss");
        // authorization check for only the owner
        impermenant_loss_stop_loss.authorize(self.impermenant_loss_stop_loss_owner)?;

        msg!("auth check passed");
        // The SPL Token ensures that faucet and receiver are the same type of token
        let token_a_transfer = spl_token::instruction::transfer(
            &self.impermenant_loss_stop_loss_token_a.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &self.impermenant_loss_stop_loss.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &[],
            self.amount_a,
        )?;

        // The SPL Token ensures that faucet and receiver are the same type of token
        let token_b_transfer = spl_token::instruction::transfer(
            &self.impermenant_loss_stop_loss_token_b.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &self.impermenant_loss_stop_loss.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &[],
            self.amount_b,
        )?;

        // The SPL Token ensures that faucet and receiver are the same type of token
        let add_liquidity = spl_token::instruction::transfer(
            &self.impermenant_loss_stop_loss_token_b.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &self.impermenant_loss_stop_loss.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &[],
            self.amount_b,
        )?;

        // program_id: &Pubkey,
        // amm_id: &Pubkey,
        // amm_authority: &Pubkey,
        // amm_open_orders: &Pubkey,
        // amm_target_orders: &Pubkey,
        // lp_mint_address: &Pubkey,
        // pool_coin_token_account: &Pubkey,
        // pool_pc_token_account: &Pubkey,
        // pool_withdraw_queue: &Pubkey,
        // pool_temp_lp_token_account: &Pubkey,
        // serum_program_id: &Pubkey,
        // serum_market: &Pubkey,
        // serum_coin_vault_account: &Pubkey,
        // serum_pc_vault_account: &Pubkey,
        // serum_vault_signer: &Pubkey,
        // user_lp_token_account: &Pubkey,
        // uer_coin_token_account: &Pubkey,
        // uer_pc_token_account: &Pubkey,
        // user_owner: &Pubkey,
    
        // amount: u64,

        // invoke_signed(
        //     &token_a_transfer,
        //     &[
        //         self.impermenant_loss_stop_loss_token_b.clone(),
        //         self.impermenant_loss_stop_loss.clone(),
        //         self.impermenant_loss_stop_loss_owner.clone(),
        //     ],
        //     &[&[self.faucet_owner_seed]],
        // )?;


        // call addLiquidity

        Ok(())
    }
}


struct AnyoneRemoveLiquidityContext<'a> {
    rent: Rent,
    impermenant_loss_stop_loss_token_a: &'a AccountInfo<'a>,
    impermenant_loss_stop_loss_token_b: &'a AccountInfo<'a>,
    impermenant_loss_stop_loss: &'a AccountInfo<'a>,
    impermenant_loss_stop_loss_owner: &'a AccountInfo<'a>, // signed
    amount_a: u64,
    amount_b: u64,
}

impl<'a> AnyoneRemoveLiquidityContext<'a> {
    fn process(&self) -> ProgramResult {
        let impermenant_loss_stop_loss = ImpermenantLossStopLoss::load_initialized(self.impermenant_loss_stop_loss)?;
        msg!("loaded impermenant_loss_stop_loss");
        // NO authorization check for only the owner
        const DECIMALS: u32 = 9;

        let token_a_price = chainlink::get_price(&chainlink::id(), feed_account)?;
    
        if let Some(price) = price {
            let decimal = Decimal::new(price, DECIMALS);
            msg!("Price is {}", decimal);
        } else {
            msg!("No current price");
        }
    
         // Store the price ourselves
         let mut price_data_account = PriceFeedAccount::try_from_slice(&my_account.data.borrow())?;
         price_data_account.answer = price.unwrap_or(0);
         price_data_account.serialize(&mut &mut my_account.data.borrow_mut()[..])?;
    
        require(
            (getPrice(tokenA) / getPrice(tokenB)) / (tokenAStartPrice - tokenBStartPrice) > minChangeFactor,
            "Prices haven't changed enough"
        )
        // The SPL Token ensures that faucet and receiver are the same type of token
        let token_a_transfer = spl_token::instruction::transfer(
            &self.impermenant_loss_stop_loss_token_a.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &self.impermenant_loss_stop_loss.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &[],
            self.amount_a,
        )?;

        // The SPL Token ensures that faucet and receiver are the same type of token
        let token_b_transfer = spl_token::instruction::transfer(
            &self.impermenant_loss_stop_loss_token_b.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &self.impermenant_loss_stop_loss.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &[],
            self.amount_b,
        )?;

        // The SPL Token ensures that faucet and receiver are the same type of token
        let add_liquidity = spl_token::instruction::transfer(
            &self.impermenant_loss_stop_loss_token_b.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &self.impermenant_loss_stop_loss.key,
            &self.impermenant_loss_stop_loss_owner.key,
            &[],
            self.amount_b,
        )?;

        // program_id: &Pubkey,
        // amm_id: &Pubkey,
        // amm_authority: &Pubkey,
        // amm_open_orders: &Pubkey,
        // amm_target_orders: &Pubkey,
        // lp_mint_address: &Pubkey,
        // pool_coin_token_account: &Pubkey,
        // pool_pc_token_account: &Pubkey,
        // pool_withdraw_queue: &Pubkey,
        // pool_temp_lp_token_account: &Pubkey,
        // serum_program_id: &Pubkey,
        // serum_market: &Pubkey,
        // serum_coin_vault_account: &Pubkey,
        // serum_pc_vault_account: &Pubkey,
        // serum_vault_signer: &Pubkey,
        // user_lp_token_account: &Pubkey,
        // uer_coin_token_account: &Pubkey,
        // uer_pc_token_account: &Pubkey,
        // user_owner: &Pubkey,
    
        // amount: u64,

        // invoke_signed(
        //     &token_a_transfer,
        //     &[
        //         self.impermenant_loss_stop_loss_token_b.clone(),
        //         self.impermenant_loss_stop_loss.clone(),
        //         self.impermenant_loss_stop_loss_owner.clone(),
        //     ],
        //     &[&[self.faucet_owner_seed]],
        // )?;


        // call addLiquidity

        Ok(())
    }
}

/// Program state handler.
pub struct Processor {}

impl Processor {
    pub fn process<'a>(
        _program_id: &Pubkey,
        accounts: &'a [AccountInfo<'a>],
        input: &[u8],
    ) -> ProgramResult {
        let accounts = Accounts(accounts);
        let instruction =
            Instruction::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            Instruction::OwnerAddLiquidity { amount_a, amount_b } => OwnerAddLiquidityContext {
                rent: accounts.get_rent(0)?,
                impermenant_loss_stop_loss_token_a: ,
                impermenant_loss_stop_loss_token_b,
                impermenant_loss_stop_loss: accounts.get(0),
                impermenant_loss_stop_loss_owner, // signed
                amount_a,
                amount_b,
            }
            .process(),
            Instruction::AnyoneRemoveLiquidity { faucet_owner_seed } => AnyoneRemoveLiquidityContext {
                rent: accounts.get_rent(0)?,
                impermenant_loss_stop_loss_token_a: ,
                impermenant_loss_stop_loss_token_b,
                impermenant_loss_stop_loss: accounts.get(0),
                impermenant_loss_stop_loss_owner, // signed
                amount_a,
                amount_b,
            }
            .process(),
        }
    }
}

