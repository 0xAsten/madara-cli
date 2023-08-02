# Madara CLI Tool

Madara is a command-line interface (CLI) tool designed to interact with Madara, providing functionalities to call various RPC methods such as getting the block number, invoking transactions, and more.

```
git clone https://github.com/0xAsten/madara-cli
cd madara-cli
cargo build --release
```

## Usage

### Get Block Number

```
madara-cli --rpc-url http://0.0.0.0:9944 starknet-block-number
```

### Starknet Call

```
madara-cli --rpc-url http://0.0.0.0:9944 starknet-call --contract-address 0x49... --entry-point-selector 0x2e... --calldata 0x00... --block-reference latest
```

### Invoke Transaction

```
madara-cli --rpc-url http://0.0.0.0:9944 starknet-add-invoke-transaction --max-fee 0xDEADB --version 0x1 --nonce 0x0 --signature 0x0,0x0 --sender-address 0x00... --calldata 0x00...,0x36...,0x0
```

## Contributing

If you'd like to contribute, please fork the repository and create a new branch, then submit a pull request.

## License

Madara is licensed under the MIT license. See LICENSE for details.
