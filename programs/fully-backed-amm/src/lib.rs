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
        ctx.accounts.init_pool(ctx.bumps, seed)?;
        Ok(())
    }

    pub fn deposite_asset(ctx: Context<DepositAsset>, amount_a: u64, amount_b: u64) -> Result<()> {
        ctx.accounts.deposite(amount_a, amount_b)?;
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, is_a: bool, amount: u64, min_slippage: u64) -> Result<()> {
        ctx.accounts.swap(is_a, amount, min_slippage)?;
        Ok(())
    }

    pub fn withdraw_asset(ctx: Context<Withdraw>, lp_amount: u64) -> Result<()> {
        ctx.accounts.withdraw(lp_amount)?;
        Ok(())
    }
}

// ++++++++++++++ AMM Workflow ++++++++++++++
// - Initialize the AMM Pool
// - Deposite assets into the AMM Pool(For first time calculate Liquidity k = Sqaure Root of XY)
// - Swap tokens
// - Withdraw assets from the AMM Pool.
// - There is no fees in swapping in this program

//+++++++++ Todos ++++++++++
// - Implement slippage protection
// - Implement Fees collection if possible it
