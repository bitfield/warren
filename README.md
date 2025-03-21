[![Crate](https://img.shields.io/crates/v/warren.svg)](https://crates.io/crates/warren)
[![Docs](https://docs.rs/warren/badge.svg)](https://docs.rs/warren)
![CI](https://github.com/bitfield/warren/actions/workflows/ci.yml/badge.svg)
![Audit](https://github.com/bitfield/warren/actions/workflows/audit.yml/badge.svg)
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# warren

A tool for value investors to analyse stock prices and determine if a given stock is currently “cheap”—that is, if its current price is below the median of its 90-day price history.

![](warren.jpg)
(Photo by [Fortune Live Media](https://www.flickr.com/photos/fortunelivemedia/10311228024), licensed under Creative Commons)

# Installation

```sh
cargo install warren
```

# Usage

Run the `warren` command with a stock ticker symbol (as understood by the [Yahoo! Finance](https://finance.yahoo.com/lookup/) API):

```sh
warren AAPL
```

```
AAPL: Current 215.05 Low 208.42 High 260.10 Position 12.83% - Buy
```

This output shows that AAPL's most recent closing price was 215.05, its 90-day low was 208.42, and its 90-day high was 260.10. This puts the current price at 12.83% of its historic range, and it is thus considered a “Buy” (if it were above 50%, it would instead be “Don't buy”).
