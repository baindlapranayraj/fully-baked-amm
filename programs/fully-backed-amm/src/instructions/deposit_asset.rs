use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
    },
};

use crate::{
    constant::{MINT_LP, POOL},
    helper::LiquidityPool,
    state::PoolConfig,
};

#[derive(Accounts)]
pub struct DepositAsset<'info> {
    #[account(mut)]
    pub liquid_provider: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = liquid_provider,
    )]
    pub provider_token_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = liquid_provider,
    )]
    pub provider_token_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = liquid_provider,
        associated_token::mint = mint_lp,
        associated_token::authority = liquid_provider,
    )]
    pub provider_lp_token: Box<InterfaceAccount<'info, TokenAccount>>,

    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        seeds = [POOL, pool_config_account.seed.to_le_bytes().as_ref()],
        bump = pool_config_account.pool_bump,
        has_one = mint_a.key(),
        has_one = mint_b.key(),
    )]
    pub pool_config_account: Box<Account<'info, PoolConfig>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = pool_config_account,
    )]
    pub vault_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = pool_config_account,
    )]
    pub vault_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        mint::authority = pool_config_account,
        mint::decimals = 6,
        seeds = [MINT_LP, pool_config_account.key().to_bytes().as_ref()],
        bump = pool_config_account.pool_mint_bump
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> DepositAsset<'info> {
    pub fn deposite(&mut self, amount_a: u64, amount_b: u64) -> Result<()> {
        let lp_token_amount = LiquidityPool::calculate_liquidity(LiquidityPool {
            total_amount_a: self.vault_a.amount,
            total_amount_b: self.vault_b.amount,
            deposit_amount_a: amount_a,
            deposit_amount_b: amount_b,
            mint_supply: self.mint_lp.supply,
        })?;
        self.deposite_token(true, amount_a)?;
        self.deposite_token(false, amount_b)?;
        self.mint_token(lp_token_amount)?;

        Ok(())
    }

    fn deposite_token(&mut self, is_a: bool, amount: u64) -> Result<()> {
        let program = self.token_program.to_account_info();
        let mint;

        let (from, to) = if is_a {
            mint = self.mint_a.clone();
            (
                self.provider_token_a.to_account_info(),
                self.vault_a.to_account_info(),
            )
        } else {
            mint = self.mint_b.clone();
            (
                self.provider_token_b.to_account_info(),
                self.vault_b.to_account_info(),
            )
        };

        let accounts = TransferChecked {
            from,
            mint: mint.to_account_info(),
            to,
            authority: self.liquid_provider.to_account_info(),
        };

        let ctx = CpiContext::new(program, accounts);
        transfer_checked(ctx, amount, mint.decimals)?;
        Ok(())
    }

    fn mint_token(&mut self, amount: u64) -> Result<()> {
        let accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.provider_lp_token.to_account_info(),
            authority: self.pool_config_account.to_account_info(),
        };

        let pool_seed = self.pool_config_account.seed.to_le_bytes();
        let seeds = [POOL, pool_seed.as_ref(), &[self.pool_config_account.pool_bump]];
        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        mint_to(ctx, amount)?;
        Ok(())
    }
}
