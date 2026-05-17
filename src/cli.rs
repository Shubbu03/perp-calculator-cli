use clap::{Parser, ValueEnum};

use crate::calculator::{PerpInput, Side};

#[derive(Parser)]
#[command(
    name = "perp-calculator",
    about = "Calculate perpetual futures PnL, ROE, and liquidation price"
)]
pub struct Cli {
    #[arg(value_enum)]
    side: CliSide,
    collateral: f64,
    leverage: f64,
    entry_price: f64,
    current_price: f64,
    maintenance_margin_bps: f64,
}

impl Cli {
    pub fn into_perp_input(self) -> PerpInput {
        PerpInput {
            collateral: self.collateral,
            leverage: self.leverage,
            entry_price: self.entry_price,
            current_price: self.current_price,
            maintenance_margin_bps: self.maintenance_margin_bps,
            side: self.side.into(),
        }
    }
}

#[derive(Clone, ValueEnum)]
enum CliSide {
    Long,
    Short,
}

impl From<CliSide> for Side {
    fn from(side: CliSide) -> Self {
        match side {
            CliSide::Long => Side::Long,
            CliSide::Short => Side::Short,
        }
    }
}
