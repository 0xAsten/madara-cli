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

Response:

```
Number(6219)
```

### Starknet Call

```
./target/debug/madara_cli call-contract --contract-address 0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7 --entry-point-selector 0x2e4263afad30923c891518314c3c95dbe830a16874e8abc5777a9a20b54c76e --calldata 0x0000000000000000000000000000000000000000000000000000000000000002
```

Response:

```
Array [
    String("0xffffffffffffffffffffffffffffffff"),
    String("0x0"),
]
```

### Invoke Transaction

```
./target/debug/madara_cli invoke-tx --signature 0x0,0x0 --sender-address 0x0000000000000000000000000000000000000000000000000000000000000001 --calldata 0x0000000000000000000000000000000000000000000000000000000000001111,0x36fa6de2810d05c3e1a0ebe23f60b9c2f4629bbead09e5a9704e1c5632630d5,0x0
```

Response:

```
Object {
    "transaction_hash": String("0x3024c929c1bec3a872441d8ad6a8e5b0c5b931d2bf75dc5dbad2b93c1c4dec0"),
}
```

## Contributing

If you'd like to contribute, please fork the repository and create a new branch, then submit a pull request.

## License

Madara is licensed under the MIT license. See LICENSE for details.
