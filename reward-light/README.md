# reward-light

A tool for collecting and analyzing Autonomys block rewards. It fetches all block reward events for the last X days and exports both detailed and summary CSV reports.

## Features

- Connects to a Substrate node to fetch block reward events.
- Supports custom node URLs and query durations.
- Exports detailed reward records as CSV.
- Aggregates rewards by account and exports a summary CSV.

## Usage

### 1. Build

```sh
cargo build --release
```

### 2. Run

```sh
./target/release/reward-light --days 7 --ws-url ws://127.0.0.1:9944 --reward-detail-csv reward_detail.csv --summary-csv sum_reward.csv
```

**Arguments:**

- `--days` Number of days to query (default: 7).
- `--ws-url` Node WebSocket URL (default: `ws://127.0.0.1:9944`).
- `--reward-detail-csv` Output file for detailed rewards (default: `reward_detail.csv`).
- `--summary-csv` Output file for account summary (default: `sum_reward.csv`).

### 3. Output

- **Reward Detail CSV:** Each record includes block number, block hash, account, and reward amount.
- **Account Summary CSV:** For each account, shows reward count, total reward, and ratio of total rewards.

## Code Structure

- `src/main.rs`: Entry point, argument parsing, and workflow.
- `src/sub_api.rs`: Blockchain API interactions, event fetching.
- `src/sub_rpc.rs`: RPC helpers for block hashes, etc.
- `src/util.rs`: Data structures, CSV export, and aggregation utilities.

## Dependencies

- [subxt](https://crates.io/crates/subxt)
- [subxt-rpcs](https://crates.io/crates/subxt-rpcs)
- [clap](https://crates.io/crates/clap)
- [serde](https://crates.io/crates/serde)
- [hex](https://crates.io/crates/hex)

---

For customization or extension, see the source code in each module.