# Madara CLI Tool

`madara-cli` is a command-line interface (CLI) tool designed to interact with Madara, providing functionalities to call various RPC methods such as getting the block number, invoking transactions, and more.

Before using the Madara CLI tool, you need to ensure that a Madara node is running. The instructions to run the Madara node are specific to your particular setup, so please refer to the relevant Madara documentation to get started.

Once the Madara node is up and running, you can interact with it using the Madara CLI tool. By default, the CLI tool will attempt to connect to a Madara node running at http://0.0.0.0:9944. If your node is running at this address, you can omit the --rpc-url argument. Otherwise, you will need to provide the correct URL using the --rpc-url option.

## Building the Project

```
git clone https://github.com/0xAsten/madara-cli
cd madara-cli
cargo build
```

## Usage

### Get The Latest Block Number

```
./target/debug/madara_cli get-blocknumber
```

### Starknet Call

```
./target/debug/madara_cli starknet-call --contract-address 0x49... --entry-point-selector 0x2e... --calldata 0x00... --block-reference latest
```

### Invoke Transaction

```
./target/debug/madara_cli starknet-add-invoke-transaction --max-fee 0xDEADB --version 0x1 --nonce 0x0 --signature 0x0,0x0 --sender-address 0x00... --calldata 0x00...,0x36...,0x0
```

## Contributing

If you'd like to contribute, please fork the repository and create a new branch, then submit a pull request.

## License

Madara is licensed under the MIT license. See LICENSE for details.
