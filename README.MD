# Blockchain

## Usage
Run to pull transfer logs for a certain contract address from a latest block:

```bash
cargo run -- -a <contract_address>
```

Run to pull transfer logs for a certain contract address starting from a certain block:

```bash
cargo run -- -a <contract_address> -b <block_number>
```

Run to start a web-server without pulling transfer logs:

```bash
cargo run
```
