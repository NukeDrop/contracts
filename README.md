## Compile

```bash
forc build --release
```

## Test

```bash
cargo test
```

## Script

### Deployment of the contract

```bash
cargo run deploy <fee_amount> <fee_address>
```

example:
```bash
cargo run deploy 10 0x2c98cd5965a7c1d9c7fb2b5e8bbdc6e7488052015335238e
da4c8bc9d8b521bb
```

### Fee info update by the owner of the contract

```bash
cargo run -- set-fee-info <contract_id> <fee_amount> <fee_address>
```

example:
```bash
cargo run set-fee-info 8010f96aac5d8cd70816574a6b8b3bc915269c7f8df5a53
07e56efb3681bd429  10 0x2c98cd5965a7c1d9c7fb2b5e8bbdc6e7488052015335238eda4c8bc9d8b521bb
```