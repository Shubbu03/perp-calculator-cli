use crate::calculator::{PerpInput, PerpResult};

pub fn print_input(input: &PerpInput) {
    println!("Perp Calculator");
    println!("===============");
    println!("Side:                   {}", input.side);
    println!("Collateral:             {:.2}", input.collateral);
    println!("Leverage:               {:.2}x", input.leverage);
    println!("Entry Price:            {:.2}", input.entry_price);
    println!("Current Price:          {:.2}", input.current_price);
    println!(
        "Maintenance Margin:     {:.2} bps",
        input.maintenance_margin_bps
    );
    println!();
}

pub fn print_result(result: &PerpResult) {
    println!("Result");
    println!("======");
    println!("Position Size:          {:.2}", result.position_size);
    println!("PnL:                    {:.2}", result.pnl);
    println!("ROE:                    {:.2}%", result.roe_percent);
    println!("Liquidation Price:      {:.2}", result.liquidation_price);
}
