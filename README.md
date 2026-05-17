# Perp Calculator

A small Rust CLI app for calculating perpetual futures position size, PnL, ROE, and liquidation price.

## Usage

```bash
cargo run -- <side> <collateral> <leverage> <entry_price> <current_price> <maintenance_margin_bps>
```

`side` must be either `long` or `short`.

## Examples

Long position:

```bash
cargo run -- long 100 5 100 110 500
```

Short position:

```bash
cargo run -- short 100 5 100 90 500
```

Show CLI help:

```bash
cargo run -- --help
```

## Development

Format and test the project:

```bash
cargo fmt
cargo test
```
