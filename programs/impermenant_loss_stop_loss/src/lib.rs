use anchor_lang::prelude::borsh::{BorshDeserialize, BorshSerialize};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::Instruction,
    program::{invoke_signed},
    program_error::ProgramError,
    program_option::COption,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use anchor_spl::token::{self, Mint, TokenAccount, Transfer};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[program]
pub mod impermenant_loss_stop_loss {
    use super::*;
    pub fn initialize_impermenant_loss_stop_loss(ctx: Context<InitializeImpermenantLossStopLoss>, nonce: u8, min_change_factor: u64) -> ProgramResult {
        msg!("initialize_impermenant_loss_stop_loss");
        let impermenant_loss_stop_loss = &mut ctx.accounts.impermenant_loss_stop_loss;

        impermenant_loss_stop_loss.nonce = nonce;
        impermenant_loss_stop_loss.min_change_factor = min_change_factor;

        impermenant_loss_stop_loss.impermenant_loss_stop_loss_user_info_account = *ctx.accounts.impermenant_loss_stop_loss_user_info_account.key;

        let seeds = &[
            ctx.accounts.impermenant_loss_stop_loss.to_account_info().key.as_ref(),
            &[ctx.accounts.impermenant_loss_stop_loss.nonce],
        ];
        let signer = &[&seeds[..]];
        let accounts = [
            ctx.accounts.raydium_pool_id.clone(),
            ctx.accounts.raydium_pool_authority.clone(),
            ctx.accounts.impermenant_loss_stop_loss_user_info_account.clone(),
            ctx.accounts.impermenant_loss_stop_loss_signer.clone(),
            ctx.accounts.impermenant_loss_stop_loss_lp_token_account.to_account_info(),
            ctx.accounts.raydium_lp_token_account.to_account_info(),
            ctx.accounts.impermenant_loss_stop_loss_token_account.to_account_info(),
            ctx.accounts.raydium_token_account.to_account_info(),
            ctx.accounts.clock.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.impermenant_loss_stop_loss_token_account_b.to_account_info(),
            ctx.accounts
                .raydium_token_account_b
                .to_account_info(),
        ];
        let account_metas = accounts
            .iter()
            .map(|acc| {
                if acc.key == ctx.accounts.impermenant_loss_stop_loss_signer.key {
                    AccountMeta::new_readonly(*acc.key, true)
                } else if acc.key == ctx.accounts.clock.to_account_info().key {
                    AccountMeta::new_readonly(*acc.key, false)
                } else {
                    AccountMeta::new(*acc.key, false)
                }
            })
            .collect::<Vec<_>>();

        // check if pool exists
        // let ix = Instruction::new_with_borsh(
        //     *ctx.accounts.raydium_pool_id.key,
        //     &DepositData {
        //         instruction: 1,
        //         amount: 0,
        //     },
        //     account_metas,
        // );
        // msg!("invoking raydium");
        // invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }

    pub fn owner_add_liquidity(
        ctx: Context<ProvideLiquidity>,
        amount: u64,
        amount_b: u64,
    ) -> ProgramResult {
        msg!("provide liquidity");
        // need to do auth check to make sure user calling this function is owner
        msg!("transfer user token A to liquidity wrapper");
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_signer.to_account_info(),
            to: ctx.accounts.impermenant_loss_stop_loss_token_account.to_account_info(),
            authority: ctx.accounts.user_signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        msg!("transfered user token A to liquidity wrapper: success!");
        msg!("transfer user token B to liquidity wrapper");
        let cpi_accounts_b = Transfer {
            from: ctx.accounts.user_signer.to_account_info(),
            to: ctx.accounts.impermenant_loss_stop_loss_token_account_b.to_account_info(),
            authority: ctx.accounts.user_signer.to_account_info(),
        };
        let cpi_program_b = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_b = CpiContext::new(cpi_program_b, cpi_accounts_b);
        token::transfer(cpi_ctx_b, amount_b)?;
        msg!("transfered user token B to liquidity wrapper: success!");
        msg!("provide liquidity raydium");
        let accounts = [
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.raydium_amm_id.clone(),
            ctx.accounts.raydium_amm_authority.clone(),
            ctx.accounts.raydium_amm_open_orders.clone(),
            ctx.accounts.raydium_amm_target_orders.clone(),
            ctx.accounts.raydium_lp_token_mint_address.to_account_info(),
            ctx.accounts.raydium_token_account.to_account_info(),
            ctx.accounts
                .raydium_token_account_b
                .to_account_info(),
            ctx.accounts.serum_market.clone(),
            ctx.accounts.impermenant_loss_stop_loss_token_account.to_account_info(),
            ctx.accounts.impermenant_loss_stop_loss_token_account_b.to_account_info(),
            ctx.accounts.impermenant_loss_stop_loss_lp_token_account.to_account_info(),
            ctx.accounts.impermenant_loss_stop_loss_signer.clone(),
        ];
        let account_metas = accounts
            .iter()
            .map(|acc| {
                if acc.key == ctx.accounts.impermenant_loss_stop_loss_signer.key {
                    AccountMeta::new_readonly(*acc.key, true)
                } else {
                    AccountMeta::new(*acc.key, false)
                }
            })
            .collect::<Vec<_>>();
        let seeds = &[
            ctx.accounts.impermenant_loss_stop_loss.to_account_info().key.as_ref(),
            &[ctx.accounts.impermenant_loss_stop_loss.nonce],
        ];
        let signer = &[&seeds[..]];
        let ix = Instruction::new_with_borsh(
            *ctx.accounts.raydium_amm_program.key,
            &ProvideLiquidityData {
                instruction: 3,
                max_coin_amount: amount,
                max_pc_amount: amount_b,
                base_side: 1,
            },
            account_metas,
        );
        msg!("invoking raydium");
        invoke_signed(&ix, &accounts, signer)?;

        Ok(())
    }

    pub fn owner_remove_liquidity(
        ctx: Context<ProvideLiquidity>,
        amount: u64,
    ) -> ProgramResult {
        msg!("owner remove liquidity");
        // need to do auth check to make sure user calling this function is owner
        let accounts = [
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.raydium_amm_id.clone(),
            ctx.accounts.raydium_amm_authority.clone(),
            ctx.accounts.raydium_amm_open_orders.clone(),
            ctx.accounts.raydium_amm_target_orders.clone(),
            ctx.accounts.raydium_lp_token_mint_address.to_account_info(),
            ctx.accounts.raydium_token_account.to_account_info(),
            ctx.accounts
                .raydium_token_account_b
                .to_account_info(),
            ctx.accounts.serum_market.clone(),
            ctx.accounts.impermenant_loss_stop_loss_token_account.to_account_info(),
            ctx.accounts.impermenant_loss_stop_loss_token_account_b.to_account_info(),
            ctx.accounts.impermenant_loss_stop_loss_lp_token_account.to_account_info(),
            ctx.accounts.impermenant_loss_stop_loss_signer.clone(),
        ];
        let account_metas = accounts
            .iter()
            .map(|acc| {
                if acc.key == ctx.accounts.impermenant_loss_stop_loss_signer.key {
                    AccountMeta::new_readonly(*acc.key, true)
                } else {
                    AccountMeta::new(*acc.key, false)
                }
            })
            .collect::<Vec<_>>();
        let seeds = &[
            ctx.accounts.impermenant_loss_stop_loss.to_account_info().key.as_ref(),
            &[ctx.accounts.impermenant_loss_stop_loss.nonce],
        ];
        let signer = &[&seeds[..]];
        let ix = Instruction::new_with_borsh(
            *ctx.accounts.raydium_amm_program.key,
            &WithdrawData {
                instruction: 3,
                amount: amount
            },
            account_metas,
        );
        msg!("invoking withdraw instruction raydium");
        invoke_signed(&ix, &accounts, signer)?;
        // need to grab output amounts and make two transfers back to user
        msg!("transfer user token A from liquidity wrapper");
        let cpi_accounts = Transfer {
            from: ctx.accounts.impermenant_loss_stop_loss_token_account.to_account_info(),
            to: ctx.accounts.user_signer.to_account_info(),
            authority: ctx.accounts.user_signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        // need to find output amount_a
        // token::transfer(cpi_ctx, amount)?;
        msg!("transfered user token A from liquidity wrapper: success!");
        msg!("transfer user token B from liquidity wrapper");
        let cpi_accounts_b = Transfer {
            from: ctx.accounts.user_signer.to_account_info(),
            to: ctx.accounts.impermenant_loss_stop_loss_token_account_b.to_account_info(),
            authority: ctx.accounts.user_signer.to_account_info(),
        };
        let cpi_program_b = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_b = CpiContext::new(cpi_program_b, cpi_accounts_b);
        // need to find output amount_b
        // token::transfer(cpi_ctx_b, amount_b)?;
        msg!("transfered user token B from liquidity wrapper: success!");
        msg!("provide liquidity raydium");


        Ok(())
    }

    pub fn anyone_remove_liquidity(
        ctx: Context<AnyoneRemoveLiquidity>,
        // this wont be needed if the amount is persisted to an acct
        amount: u64
    ) -> ProgramResult {
        msg!("anyone remove liquidity");
        // need to check output amounts have deviated from min_change_factor
        let accounts = [
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.raydium_amm_id.clone(),
            ctx.accounts.raydium_amm_authority.clone(),
            ctx.accounts.raydium_amm_open_orders.clone(),
            ctx.accounts.raydium_amm_target_orders.clone(),
            ctx.accounts.raydium_lp_token_mint_address.to_account_info(),
            ctx.accounts.raydium_token_account.to_account_info(),
            ctx.accounts
                .raydium_token_account_b
                .to_account_info(),
            ctx.accounts.serum_market.clone(),
            ctx.accounts.impermenant_loss_stop_loss_token_account.to_account_info(),
            ctx.accounts.impermenant_loss_stop_loss_token_account_b.to_account_info(),
            ctx.accounts.impermenant_loss_stop_loss_lp_token_account.to_account_info(),
            ctx.accounts.impermenant_loss_stop_loss_signer.clone(),
        ];
        let account_metas = accounts
            .iter()
            .map(|acc| {
                if acc.key == ctx.accounts.impermenant_loss_stop_loss_signer.key {
                    AccountMeta::new_readonly(*acc.key, true)
                } else {
                    AccountMeta::new(*acc.key, false)
                }
            })
            .collect::<Vec<_>>();
        let seeds = &[
            ctx.accounts.impermenant_loss_stop_loss.to_account_info().key.as_ref(),
            &[ctx.accounts.impermenant_loss_stop_loss.nonce],
        ];
        let signer = &[&seeds[..]];
        let ix = Instruction::new_with_borsh(
            *ctx.accounts.raydium_amm_program.key,
            &WithdrawData {
                instruction: 3,
                amount: amount
            },
            account_metas,
        );
        msg!("invoking withdraw instruction raydium");
        invoke_signed(&ix, &accounts, signer)?;
        // if amounts have deviated from min_change_factor, then continue execution of instructions
        // need to grab output amounts and make two transfers back to user
        msg!("transfer user token A from liquidity wrapper");
        let cpi_accounts = Transfer {
            from: ctx.accounts.impermenant_loss_stop_loss_token_account.to_account_info(),
            to: ctx.accounts.user_signer.to_account_info(),
            authority: ctx.accounts.user_signer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        // need to find output amount_a
        // token::transfer(cpi_ctx, amount)?;
        msg!("transfered user token A from liquidity wrapper: success!");
        msg!("transfer user token B from liquidity wrapper");
        let cpi_accounts_b = Transfer {
            from: ctx.accounts.user_signer.to_account_info(),
            to: ctx.accounts.impermenant_loss_stop_loss_token_account_b.to_account_info(),
            authority: ctx.accounts.user_signer.to_account_info(),
        };
        let cpi_program_b = ctx.accounts.token_program.to_account_info();
        let cpi_ctx_b = CpiContext::new(cpi_program_b, cpi_accounts_b);
        // need to find output amount_b
        // token::transfer(cpi_ctx_b, amount_b)?;
        msg!("transfered user token B from liquidity wrapper: success!");
        msg!("provide liquidity raydium");


        Ok(())
    }
}


#[derive(Accounts)]
pub struct ProvideLiquidity<'info> {
    pub impermenant_loss_stop_loss: ProgramAccount<'info, InitializeImpermenantLossStopLossAccount>,
    #[account(seeds = [impermenant_loss_stop_loss.to_account_info().key.as_ref(), &[impermenant_loss_stop_loss.nonce]])]
    pub impermenant_loss_stop_loss_signer: AccountInfo<'info>,
    #[account(mut)]
    pub impermenant_loss_stop_loss_lp_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub impermenant_loss_stop_loss_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub impermenant_loss_stop_loss_token_account_b: CpiAccount<'info, TokenAccount>,
    //user
    #[account(mut, signer)]
    pub user_signer: AccountInfo<'info>,
    // raydium
    pub raydium_amm_program: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_amm_id: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_amm_authority: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_amm_open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_amm_target_orders: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_lp_token_mint_address: CpiAccount<'info, Mint>,
    #[account(mut)]
    pub raydium_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub raydium_token_account_b: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    #[account(mut, "token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct OwnerRemoveLiquidity<'info> {
    pub impermenant_loss_stop_loss: ProgramAccount<'info, InitializeImpermenantLossStopLossAccount>,
    #[account(seeds = [impermenant_loss_stop_loss.to_account_info().key.as_ref(), &[impermenant_loss_stop_loss.nonce]])]
    pub impermenant_loss_stop_loss_signer: AccountInfo<'info>,
    #[account(mut)]
    pub impermenant_loss_stop_loss_lp_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub impermenant_loss_stop_loss_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub impermenant_loss_stop_loss_token_account_b: CpiAccount<'info, TokenAccount>,
    //user
    #[account(mut, signer)]
    pub user_signer: AccountInfo<'info>,
    // raydium
    pub raydium_amm_program: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_amm_id: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_amm_authority: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_amm_open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_amm_target_orders: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_lp_token_mint_address: CpiAccount<'info, Mint>,
    #[account(mut)]
    pub raydium_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub raydium_token_account_b: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    #[account(mut, "token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AnyoneRemoveLiquidity <'info> {
    pub impermenant_loss_stop_loss: ProgramAccount<'info, InitializeImpermenantLossStopLossAccount>,
    #[account(seeds = [impermenant_loss_stop_loss.to_account_info().key.as_ref(), &[impermenant_loss_stop_loss.nonce]])]
    pub impermenant_loss_stop_loss_signer: AccountInfo<'info>,
    #[account(mut)]
    pub impermenant_loss_stop_loss_lp_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)] 
    pub impermenant_loss_stop_loss_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub impermenant_loss_stop_loss_token_account_b: CpiAccount<'info, TokenAccount>,
    // user signer is not needed as anyone can call this function (if user_signer AccountInfo is needed, fetch it from store)
    #[account(mut, signer)]
    pub user_signer: AccountInfo<'info>,
    // raydium
    pub raydium_amm_program: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_amm_id: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_amm_authority: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_amm_open_orders: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_amm_target_orders: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_lp_token_mint_address: CpiAccount<'info, Mint>,
    #[account(mut)]
    pub raydium_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub raydium_token_account_b: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    #[account(mut, "token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeImpermenantLossStopLoss<'info> {
    // vault
    #[account(init)]
    pub impermenant_loss_stop_loss: ProgramAccount<'info, InitializeImpermenantLossStopLossAccount>,
    pub impermenant_loss_stop_loss_signer: AccountInfo<'info>,

    #[account(mut)]
    pub impermenant_loss_stop_loss_user_info_account: AccountInfo<'info>,
    #[account(mut)]
    pub impermenant_loss_stop_loss_lp_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub impermenant_loss_stop_loss_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub impermenant_loss_stop_loss_token_account_b: CpiAccount<'info, TokenAccount>,
    // raydium
    #[account(mut)]
    pub raydium_pool_id: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_pool_authority: AccountInfo<'info>,
    #[account(mut)]
    pub raydium_lp_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub raydium_token_account: CpiAccount<'info, TokenAccount>,
    #[account(mut)]
    pub raydium_token_account_b: CpiAccount<'info, TokenAccount>,
    #[account(mut, "token_program.key == &token::ID")]
    pub token_program: AccountInfo<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct ProvideLiquidityData {
    pub instruction: u8,
    pub max_coin_amount: u64,
    pub max_pc_amount: u64,
    pub base_side: u64,
}

#[account]
pub struct InitializeImpermenantLossStopLossAccount {
    // ilsl account
    pub nonce: u8,
    pub impermenant_loss_stop_loss_user_info_account: Pubkey,
    pub min_change_factor: u64
}


#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct DepositData {
    pub instruction: u8,
    pub amount: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct WithdrawData {
    pub instruction: u8,
    pub amount: u64,
}

pub struct RaydiumUserInfoAccount {
    pub state: u64,
    pub pool_id: Pubkey,
    pub staker_owner: Pubkey,
    pub deposit_balance: u64,
    pub reward_debt: u64,
    pub reward_debt_b: u64,
}

impl Sealed for RaydiumUserInfoAccount {}

impl IsInitialized for RaydiumUserInfoAccount {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Pack for RaydiumUserInfoAccount {
    const LEN: usize = 96;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, RaydiumUserInfoAccount::LEN];
        // TODO: Update this to remove staker_owner, update reward_debt vars with token_amount
        let (state, pool_id, staker_owner, deposit_balance, reward_debt, reward_debt_b) =
            array_refs![src, 8, 32, 32, 8, 8, 8];

        Ok(RaydiumUserInfoAccount {
            state: u64::from_le_bytes(*state),
            pool_id: Pubkey::new_from_array(*pool_id),
            staker_owner: Pubkey::new_from_array(*staker_owner),
            deposit_balance: u64::from_le_bytes(*deposit_balance),
            reward_debt: u64::from_le_bytes(*reward_debt),
            reward_debt_b: u64::from_le_bytes(*reward_debt_b),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, RaydiumUserInfoAccount::LEN];
        let (
            state_dst,
            pool_id_dst,
            staker_owner_dst,
            deposit_balance_dst,
            reward_debt_dst,
            reward_debt_b_dst,
        ) = mut_array_refs![dst, 8, 32, 32, 8, 8, 8];

        let RaydiumUserInfoAccount {
            state,
            pool_id,
            staker_owner,
            deposit_balance,
            reward_debt,
            reward_debt_b,
        } = self;

        *state_dst = state.to_le_bytes();
        pool_id_dst.copy_from_slice(pool_id.as_ref());
        staker_owner_dst.copy_from_slice(staker_owner.as_ref());
        *deposit_balance_dst = deposit_balance.to_le_bytes();
        *reward_debt_dst = reward_debt.to_le_bytes();
        *reward_debt_b_dst = reward_debt_b.to_le_bytes();
    }
}
