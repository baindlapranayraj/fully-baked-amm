use crate::error::AMMError;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{constant::POOL, helper::SwapToken, state::PoolConfig, swap_slippage_check};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = user
    )]
    pub user_token_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = user
    )]
    pub user_token_b: InterfaceAccount<'info, TokenAccount>,

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

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Swap<'info> {
    pub fn swap(&mut self, is_a: bool, amount: u64, min_slippage: u64) -> Result<()> {
        let send_amount = SwapToken::swap_token(SwapToken {
            is_a,
            deposit_amount: amount,
            total_amount_a: self.vault_a.amount,
            total_amount_b: self.vault_b.amount,
        })?;
        

        swap_slippage_check!(min_slippage, send_amount);

        self.deposit_tokens(is_a, amount)?;
        self.transfer_user(is_a, send_amount)?;

        Ok(())
    }

    fn deposit_tokens(&mut self, is_a: bool, amount: u64) -> Result<()> {
        let mint: InterfaceAccount<'info, Mint>;
        let (from, to) = match is_a {
            true => {
                mint = self.mint_a.clone();
                (
                    self.user_token_a.to_account_info(),
                    self.vault_a.to_account_info(),
                )
            }
            false => {
                mint = self.mint_b.clone();
                (
                    self.user_token_b.to_account_info(),
                    self.vault_b.to_account_info(),
                )
            }
        };

        let accounts = TransferChecked {
            from,
            mint: mint.to_account_info(),
            to,
            authority: self.user.to_account_info(),
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        transfer_checked(ctx, amount, mint.decimals)?;
        Ok(())
    }

    fn transfer_user(&mut self, is_a: bool, amount: u64) -> Result<()> {
        let mint: InterfaceAccount<'info, Mint>;
        let (from, to) = match is_a {
            true => {
                mint = self.mint_b.clone();
                (
                    self.vault_b.to_account_info(),
                    self.user_token_b.to_account_info(),
                )
            }
            false => {
                mint = self.mint_a.clone();
                (
                    self.vault_a.to_account_info(),
                    self.user_token_a.to_account_info(),
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
}
