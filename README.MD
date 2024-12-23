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

### Endpoints

`GET http://localhost:3000/blocks` - get blocks

`GET http://localhost:3000/blocks?start_hash=<hash>` - get blocks starting from a certain hash

`GET http://localhost:3000/blocks/remote/<hex_url>` - get blocks from a remote server using hex-encoded url(url:port/blocks)

`GET http://localhost:3000/blocks/remote/<hex_url>?start_hash=<hash>` - get blocks from a remote server using hex-encoded url(url:port/blocks) starting from a certain hash

