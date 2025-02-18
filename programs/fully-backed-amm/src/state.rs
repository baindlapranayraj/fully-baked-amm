use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PoolConfig {
    pub owner: Option<Pubkey>,

    pub mint_a: Pubkey,
    pub mint_b: Pubkey,

    pub vault_a_bump:u8,
    pub vault_b_bump:u8,

    pub seed: u64,
    pub pool_mint_bump: u8,
    pub pool_bump: u8,
}
