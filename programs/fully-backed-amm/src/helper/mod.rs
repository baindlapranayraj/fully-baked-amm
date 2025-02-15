use anchor_lang::prelude::*;

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
