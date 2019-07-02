# process-usage

A Rust crate used to process the user_app_usage table.

## Data

To use this crate, please edit `src/main.rs` and change the path to the `user_app_usage.csv` file(line 86) to where your usage file is placed. 

The program will also look for `app_info.csv` and `user_basic_info.csv` under `../data/` directory relative to the current path.

The output `usages_summary.csv` will be placed under `../data/`

## Run

The compiler used was `rustc 1.35.0`

```shell
cargo run --release
```