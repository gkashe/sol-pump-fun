use std::ops::{Div, Mul};

pub fn calc_swap_quote(
    in_amount: u64,
    real_sol_reserves: u64,
    bonding_curve_slope: u64,
    input_sol: bool,
) -> u64 {
    if input_sol {
        ((in_amount as f64)
            .div(real_sol_reserves as f64)
            .mul(bonding_curve_slope as f64)) as u64
    } else {
        ((in_amount as f64)
            .div(bonding_curve_slope as f64)
            .mul(real_sol_reserves as f64)) as u64
    }
}
