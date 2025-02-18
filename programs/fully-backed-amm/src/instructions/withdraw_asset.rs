use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{burn, transfer_checked, Burn, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use crate::{constant::{MINT_LP, POOL}, state::PoolConfig, withdraw_token};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = user
    )]
    pub user_token_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = user
    )]
    pub user_token_b: Box<InterfaceAccount<'info, TokenAccount>>,

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
        seeds = [MINT_LP, pool_config_account.key().to_bytes().as_ref()],
        bump = pool_config_account.pool_mint_bump,
        mint::authority = pool_config_account,
        mint::token_program = token_program
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_lp,
        associated_token::authority = user
    )]
    pub user_token_lp: Box<InterfaceAccount<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, lp_amount: u64) -> Result<()> {
        let amount_a = withdraw_token!(self, true, lp_amount);
        let amount_b = withdraw_token!(self, false, lp_amount);

        self.transfer_token(true, amount_a)?;
        self.transfer_token(false, amount_b)?;
        self.burn_token(lp_amount)?;

        Ok(())
    }

    fn transfer_token(&mut self, is_a: bool, amount: u64) -> Result<()> {
        let mint: Box<InterfaceAccount<'info, Mint>>;

        let (from, to) = match is_a {
            true => {
                mint = self.mint_a.clone();
                (
                    self.vault_a.to_account_info(),
                    self.user_token_a.to_account_info(),
                )
            }
            false => {
                mint = self.mint_b.clone();
                (
                    self.vault_b.to_account_info(),
                    self.user_token_b.to_account_info(),
                )
            }
        };

        let accounts = TransferChecked {
            from,
            mint: mint.to_account_info(),
            to,
            authority: self.pool_config_account.to_account_info(),
        };

        let secret_seed = self.pool_config_account.seed.to_le_bytes();
        let seeds = &[
            POOL,
            secret_seed.as_ref(),
            &[self.pool_config_account.pool_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer_checked(ctx, amount, mint.decimals)?;

        Ok(())
    }

    fn burn_token(&mut self, amount: u64) -> Result<()> {
        let accounts = Burn {
            mint: self.mint_lp.to_account_info(),
            from: self.user_token_lp.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        burn(ctx, amount)?;

        Ok(())
    }
}

// The withdraw flow
// - calculate the withdraw amount based on the lp token given from user
// - transfer from the pool to user wallet
// - burn the lp tokens
