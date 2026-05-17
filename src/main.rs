mod calculator;
mod cli;
mod display;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    let input = cli.into_perp_input();

    display::print_input(&input);

    match calculator::calculate_perp(&input) {
        Ok(result) => display::print_result(&result),
        Err(error) => eprintln!("Error: {error}"),
    }
}
