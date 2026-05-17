use std::fmt;

#[derive(Clone, Copy)]
pub enum Side {
    Long,
    Short,
}

impl fmt::Display for Side {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::Long => write!(formatter, "Long"),
            Side::Short => write!(formatter, "Short"),
        }
    }
}

pub struct PerpInput {
    pub collateral: f64,
    pub leverage: f64,
    pub entry_price: f64,
    pub current_price: f64,
    pub maintenance_margin_bps: f64,
    pub side: Side,
}

pub struct PerpResult {
    pub position_size: f64,
    pub pnl: f64,
    pub roe_percent: f64,
    pub liquidation_price: f64,
}

fn validate_input(input: &PerpInput) -> Result<(), String> {
    if input.collateral <= 0.0 {
        return Err("collateral must be greater than 0".to_string());
    }

    if input.leverage <= 0.0 {
        return Err("leverage must be greater than 0".to_string());
    }

    if input.entry_price <= 0.0 {
        return Err("entry price must be greater than 0".to_string());
    }

    if input.current_price <= 0.0 {
        return Err("current price must be greater than 0".to_string());
    }

    if input.maintenance_margin_bps < 0.0 {
        return Err("maintenance margin bps cannot be negative".to_string());
    }

    Ok(())
}

pub fn calculate_perp(input: &PerpInput) -> Result<PerpResult, String> {
    validate_input(input)?;

    let position_size = input.collateral * input.leverage;

    let pnl = match input.side {
        Side::Long => position_size * (input.current_price - input.entry_price) / input.entry_price,
        Side::Short => {
            position_size * (input.entry_price - input.current_price) / input.entry_price
        }
    };

    let roe_percent = pnl / input.collateral * 100.0;

    let maintenance_margin = input.maintenance_margin_bps / 10_000.0;

    let liquidation_price = match input.side {
        Side::Long => input.entry_price * (1.0 - (1.0 / input.leverage) + maintenance_margin),
        Side::Short => input.entry_price * (1.0 + (1.0 / input.leverage) - maintenance_margin),
    };

    Ok(PerpResult {
        position_size,
        pnl,
        roe_percent,
        liquidation_price,
    })
}
