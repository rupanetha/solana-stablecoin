use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2, get_feed_id_from_hex};

use crate::{Collateral, Config};

// pub fn calculate_health_factor(
//     collateral: &Account<Collateral>,
//     config: &Account<Config>,
//     price_feed: &Account<PriceUpdateV2>,
// ) -> Result<u64> {
//     let collateral_value_in_usd = get_usd_value(&collateral.lamport_balance, price_feed)?;
// };

// pub fn get_usd_value(amount_in_lamports: &u64, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
//     let feed_id = get_feed_id_from_hex(input)
// }