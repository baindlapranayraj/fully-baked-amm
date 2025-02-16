use anchor_lang::prelude::*;

// ------ for deposit asset ------

#[macro_export]
macro_rules! check_asset {
    ($var:expr,$amount_a:expr,$amount_b:expr) => {
        require!(
            $var.vault_a.amount / $var.vault_b.amount
                == ($var.vault_a.amount + $amount_a) / ($var.vault_b.amount + $amount_b),
            AMMError::NotValidAsset
        )
    };
}

#[macro_export]
macro_rules! calculate_lp_tokens {
    ($var:expr,$amount_a:expr,$amount_b:expr) => {
        ($amount_a / $var.vault_a.amount) * $var.mint_lp.supply
    };
}

// ------ for swaping ------

#[macro_export]
macro_rules! swaping_a_for_b {
    ($var:expr,$deposite_a:expr) => {
        (($var.vault_b.amount as u64) * ($deposite_a as u64))
            / (($var.vault_a.amount as u64) + ($deposite_a as u64))
    };
}

#[macro_export]
macro_rules! swaping_b_for_a {
    ($var:expr,$deposite_b:expr) => {
        ($var.vault_a.amount * $deposite_b) / ($var.vault_b.amount + $deposite_b)
    };
}

// ------ for withdraw asset ------

#[macro_export]
macro_rules! withdraw_token {
// dx = X(S/T)
//
// dy = Y(S/T)
    ($var:expr,$is_a:expr,$lp_amount:expr) => {
    match $is_a {
        true => ($var.vault_a.amount * ($lp_amount / $var.mint_lp.supply)) as u64,
        false => ($var.vault_b.amount * ($lp_amount / $var.mint_lp.supply)) as u64,
    }
    };
}



// ---- Helper Functions -------