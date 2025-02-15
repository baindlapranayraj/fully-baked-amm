use anchor_lang::prelude::*;

declare_id!("AYSuzxZZj6aPpSMwHAUTGoKzG3TdroXG2TvvYyfkE2Qx");

pub mod constant;
pub mod error;
pub mod helper;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod fully_backed_amm {
    use super::*;

    pub fn initialize(ctx: Context<InitializePool>, seed: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.init_pool(ctx.bumps, seed)?;
        Ok(())
    }

    pub fn deposite_asset(ctx: Context<DepositAsset>, amount_a: u64, amount_b: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.deposite(amount_a, amount_b)?;
        Ok(())
    }

    // pub fn swap(ctx: Context<InitializePool>, seed: u8) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     ctx.accounts.init_pool(ctx.bumps, seed)?;
    //     Ok(())
    // }

    // pub fn withdraw_asset(ctx: Context<InitializePool>, seed: u8) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     ctx.accounts.init_pool(ctx.bumps, seed)?;
    //     Ok(())
    // }
}

// ++++++++++++++ AMM Workflow ++++++++++++++
// - Initialize the AMM Pool
// - Deposite assets into the AMM Pool(For first time no need to share LP tokens)
// - Swap tokens
// - Withdraw assets from the AMM Pool.
