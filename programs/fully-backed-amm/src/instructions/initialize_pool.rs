use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{constant::*, state::PoolConfig};

// ++++++ Accounts ++++++
// - admin
// - vault_a
// - vault_b
// - mint_a
// - mint_b
// - mint_lp
// - pool_config

// Lets say
// - mint_a = BONK
// - mint_b = POPCAT

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = admin,
        space = 8 + PoolConfig::INIT_SPACE ,
        seeds = [POOL,seed.to_le_bytes().as_ref()],
        bump
    )]
    pub pool_config_account: Account<'info, PoolConfig>,

    #[account(
        init,
        payer = admin,
        seeds = [MINT_LP,pool_config_account.key().to_bytes().as_ref()],
        bump,
        mint::authority = pool_config_account,
        mint::decimals = 6,
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,

    // Vault Accounts
    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_a,
        associated_token::authority = pool_config_account,
        associated_token::token_program  = associated_token_program,
    )]
    pub vault_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_b,
        associated_token::authority = pool_config_account,
        associated_token::token_program  = associated_token_program,
    )]
    pub vault_b: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializePool<'info> {
    pub fn init_pool(&mut self, bumps: InitializePoolBumps, seeds: u64) -> Result<()> {
        // Saving the pool config data

        self.pool_config_account.set_inner(PoolConfig {
            owner: Some(self.admin.key()),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            seed: seeds,
            pool_mint_bump: bumps.mint_lp,
            pool_bump: bumps.pool_config_account,
        });

        Ok(())
    }
}
