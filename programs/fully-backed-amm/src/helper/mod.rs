use crate::error::AMMError;
use anchor_lang::prelude::*;
use fixed::types::I64F64;

macro_rules! check_zero {
    ($arr:expr) => {
        if $arr.contains(&0u64) {
            return err!(AMMError::AmountZero);
        }
    };
}

#[macro_export]
macro_rules! check_asset {
    ($amount_a:expr, $amount_b:expr, $total_amount_a:expr, $total_amount_b:expr) => {
        require!(
            $total_amount_a / $total_amount_b
                == ($total_amount_a + $amount_a) / ($total_amount_b + $amount_b),
            AMMError::NotValidAsset
        );
    };
}

#[macro_export]
macro_rules! swap_slippage_check {
    ($slippage_amount:expr,$actual_amount:expr) => {
        if $slippage_amount > $actual_amount {
            return err!(AMMError::SlippageLimitExceeded);
        }
    };
}

pub struct LiquidityPool {
    pub total_amount_a: u64,
    pub total_amount_b: u64,
    pub deposit_amount_a: u64,
    pub deposit_amount_b: u64,
    pub mint_supply: u64,
}

impl LiquidityPool {
    pub fn calculate_liquidity(lp_request: LiquidityPool) -> Result<u64> {
        if lp_request.mint_supply == 0
            && lp_request.total_amount_a == 0
            && lp_request.total_amount_b == 0
        {
            let lp_amount = LiquidityPool::calculate_first_liquidity(
                lp_request.deposit_amount_a,
                lp_request.deposit_amount_b,
            )?;
            Ok(lp_amount)
        } else {
            check_asset!(
                lp_request.deposit_amount_a,
                lp_request.deposit_amount_b,
                lp_request.total_amount_a,
                lp_request.total_amount_b
            );
            let lp_amount = LiquidityPool::calculate_lp_token(lp_request)?;

            Ok(lp_amount)
        }
    }

    fn calculate_first_liquidity(amount_x: u64, amount_y: u64) -> Result<u64> {
        // K = sqrt(XY); this is given to the admin who created the pool
        // Let's say X = 1000 and Y = 1000, K = 1000 lp tokens
        check_zero!([amount_x, amount_y]);
        let liquidity = (amount_x.checked_mul(amount_y).ok_or(AMMError::Overflow)? as f64).sqrt();

        Ok(liquidity.round() as u64)
    }

    fn calculate_lp_token(lp_deposite: LiquidityPool) -> Result<u64> {
        // s = min(dx/X,dy/Y)*T
        check_zero!([
            lp_deposite.total_amount_a,
            lp_deposite.deposit_amount_a,
            lp_deposite.mint_supply
        ]);

        let lp_token = (lp_deposite.mint_supply as f64)
            * f64::min(
                (lp_deposite.deposit_amount_a as f64) / (lp_deposite.total_amount_a as f64),
                (lp_deposite.deposit_amount_b as f64) / (lp_deposite.total_amount_b as f64),
            );
        Ok(lp_token as u64)
    }
}

pub struct SwapToken {
    pub is_a: bool,
    pub deposit_amount: u64,
    pub total_amount_a: u64,
    pub total_amount_b: u64,
}

impl SwapToken {
    pub fn swap_token(swap_arg: SwapToken) -> Result<u64> {
        match swap_arg.is_a {
            true => {
                // dy = Ydx/(X + dx)
                let swap_amount = SwapToken::swap_b_for_a(swap_arg)?;
                Ok(swap_amount)
            }
            false => {
                // dx = Xdy/(Y + dy)
                let swap_amount = SwapToken::swap_a_for_b(swap_arg)?;
                Ok(swap_amount)
            }
        }
    }

    fn swap_b_for_a(swap_arg: SwapToken) -> Result<u64> {
        let total_b = I64F64::from_num(swap_arg.total_amount_b);
        let total_a = I64F64::from_num(swap_arg.total_amount_a);
        let deposit_a = I64F64::from_num(swap_arg.deposit_amount);

        // swap_amount = total_b * deposit_a / (total_a + deposit_a)
        let numerator = total_b.checked_mul(deposit_a).ok_or(AMMError::Overflow)?;
        let denominator = total_a + deposit_a;
        let swap_amount = numerator / denominator;
        Ok(swap_amount.round().to_num::<u64>()) // sending rounding number, which might cause some percision loss
    }

    fn swap_a_for_b(swap_arg: SwapToken) -> Result<u64> {
        // dx = Xdy/(Y + dy)
        let total_amount_a = I64F64::from_num(swap_arg.total_amount_a);
        let total_amount_b = I64F64::from_num(swap_arg.total_amount_b);
        let deposit_b = I64F64::from_num(swap_arg.deposit_amount);

        let numerator = total_amount_a
            .checked_mul(deposit_b)
            .ok_or(AMMError::Overflow)?;
        let denominator = total_amount_b
            .checked_add(deposit_b)
            .ok_or(AMMError::Overflow)?;

        let swap_amount = numerator / denominator;
        Ok(swap_amount.round().to_num::<u64>()) // sending rounding number, which might cause some percision loss
    }
}

pub struct WithdrawAsset {
    pub mint_supply: u64,
    pub lp_share_amount: u64,
    pub total_amount_vault: u64,
}

impl WithdrawAsset {
    // Calculate user's token A and B using lp_token_share
    // dx = A(S/T)
    // for value of each lp token to token_a is:- a = A/T
    pub fn calculate_token(withdraw_arg: WithdrawAsset) -> Result<u64> {
        check_zero!([withdraw_arg.lp_share_amount, withdraw_arg.mint_supply]);
        let lp_ratio = withdraw_arg.lp_share_amount as f64 / withdraw_arg.mint_supply as f64; // S/T

        let token_amount = (withdraw_arg.total_amount_vault as f64) * lp_ratio; //  Eg:- A(S/T)
        Ok(token_amount as u64)
    }
}
