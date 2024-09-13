# transaction-processor

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

Toy transactions processor that processes data from CSV files.

## :clipboard: Features

- Handles deposits, withdrawals, disputes, resolutions, and chargebacks.
- Rejects invalid transactions gracefully; future valid transactions are still processed.
- Includes a checkpointing system. Account and transaction data is kept in memory as transactions are processed, but after every 10 transactions, that memory is written to disk as a JSON file to allow recovery in the case of a system failure.
- Includes unit and integration tests.

## :rocket: Run

Run with Cargo. Specify an input CSV file and (optionally) an output file.

```shell
cargo run -- input.csv > output.csv
```

Run unit and integration tests:

```shell
cargo test
```
