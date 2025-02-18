use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token_interface::{Mint, TokenInterface},
};

use crate::{constant::*, state::PoolConfig};
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = admin,
        space = 8 + PoolConfig::INIT_SPACE ,
        seeds = [POOL,seed.to_le_bytes().as_ref()],
        bump
    )]
    pub pool_config_account: Box<Account<'info, PoolConfig>>,

    #[account(
        init,
        payer = admin,
        seeds = [MINT_LP,pool_config_account.key().to_bytes().as_ref()],
        bump,
        mint::authority = pool_config_account,
        mint::decimals = 6,
        mint::token_program = token_program
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,

    // Vault Accounts
    /// CHECK: Verified in CPI
    #[account(
        mut,
        seeds = [
            pool_config_account.key().as_ref(), // Owner
            token_program.key().as_ref(),       // token_program_id
            mint_a.key().as_ref(),              // mint_account_id
        ],
        seeds::program = associated_token::ID,
        bump
    )]
    pub vault_a: UncheckedAccount<'info>,

    /// CHECK: Verified in CPI
    #[account(
        mut,
        seeds = [
            pool_config_account.key().as_ref(),
            token_program.key().as_ref(),
            mint_b.key().as_ref(),
        ],
        seeds::program = associated_token::ID,
        bump
    )]
    pub vault_b: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializePool<'info> {
    pub fn init_pool(&mut self, bumps: InitializePoolBumps, seeds: u64) -> Result<()> {
        // create ATAs
        let ctx_a_accounts = associated_token::Create {
            payer: self.admin.to_account_info(),
            associated_token: self.vault_a.to_account_info(),
            authority: self.pool_config_account.to_account_info(),
            mint: self.mint_a.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };

        associated_token::create_idempotent(CpiContext::new(
            self.associated_token_program.to_account_info(),
            ctx_a_accounts,
        ))?;

        let ctx_b_accounts = associated_token::Create {
            payer: self.admin.to_account_info(),
            associated_token: self.vault_b.to_account_info(),
            authority: self.pool_config_account.to_account_info(),
            mint: self.mint_b.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        associated_token::create_idempotent(CpiContext::new(
            self.associated_token_program.to_account_info(),
            ctx_b_accounts,
        ))?;

        // Saving the pool config data
        self.pool_config_account.set_inner(PoolConfig {
            owner: Some(self.admin.key()),

            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            
            seed: seeds,
            pool_mint_bump: bumps.mint_lp,
            pool_bump: bumps.pool_config_account,
            
            vault_a_bump:bumps.vault_a,
            vault_b_bump:bumps.vault_b
        });

        Ok(())
    }
}
