use anchor_lang::prelude::*;

use crate::error::AMMError;

macro_rules! check_zero {
    ($arr:expr) => {
        if $arr.contains(&0u64) {
            return err!(AMMError::AmountZero);
        }
    };
}

// ------ for deposit asset ------

#[macro_export]
macro_rules! check_asset {
    ($var:expr, $amount_a:expr, $amount_b:expr) => {
        require!(
            $var.vault_a.amount / $var.vault_b.amount
                == ($var.vault_a.amount + $amount_a) / ($var.vault_b.amount + $amount_b),
            AMMError::NotValidAsset
        )
    };
}

#[macro_export]
macro_rules! calculate_lp_tokens {
    ($var:expr, $amount_a:expr, $amount_b:expr) => {
        if $var.vault_a.amount == 0
            || $var.vault_b.amount == 0
            || $var.mint_lp.supply == 0
        {
            return err!(AMMError::AmountZero);
        }
        ($amount_a / $var.vault_a.amount) * $var.mint_lp.supply
    };
}

// ------ for swaping ------

#[macro_export]
macro_rules! swaping_a_for_b {
    ($var:expr, $deposite_a:expr) => {
        (($var.vault_b.amount as u64) * ($deposite_a as u64))
            / (($var.vault_a.amount as u64) + ($deposite_a as u64))
    };
}

#[macro_export]
macro_rules! swaping_b_for_a {
    ($var:expr, $deposite_b:expr) => {
        ($var.vault_a.amount * $deposite_b) / ($var.vault_b.amount + $deposite_b)
    };
}

// ------ for withdraw asset ------

#[macro_export]
macro_rules! withdraw_token {
    // dx = X(S/T)
    //
    // dy = Y(S/T)
    ($var:expr, $is_a:expr, $lp_amount:expr) => {
        match $is_a {
            true => (($var.vault_a.amount * ($lp_amount / $var.mint_lp.supply)) as u64),
            false => (($var.vault_b.amount * ($lp_amount / $var.mint_lp.supply)) as u64),
        }
    };
}

// ---- Helper Functions -------

pub fn calculate_liquidity(amount_x: u64, amount_y: u64) -> Result<u64> {
    // K = sqr(XY);
    // lets say X = 1000 and Y = 1000
    // K = 1000
    check_zero!([amount_x, amount_y]);
    let liquidity = (amount_x
        .checked_mul(amount_y)
        .ok_or(AMMError::Overflow)? as f64).sqrt();
    Ok(liquidity.round() as u64)
}

pub fn calculate_lp_token(
    total_amount_x: u64,
    deposite_amount_x: u64,
    total_mint_supply: u64,
) -> Result<u64> {
    // s = (dx/X)*T
    check_zero!([total_amount_x, deposite_amount_x, total_mint_supply]);
    let lp_token = deposite_amount_x
        .checked_div(total_amount_x)
        .ok_or(AMMError::Overflow)?
        .checked_mul(total_mint_supply)
        .ok_or(AMMError::Overflow)?;
    Ok(lp_token)
}