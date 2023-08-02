use reqwest::Error;
use serde_json::{json, Value};
use std::{collections::HashMap, vec};
use structopt::StructOpt;

use madara_cli_core::{Command, Opt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let rpc_url = opt.rpc_url;
    let payload;

    match opt.command {
        Command::StarknetChainId => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_chainId",
                "params": [],
                "id": "1",
            });
        }
        Command::StarknetBlockNumber => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_blockNumber",
                "params": [],
                "id": "1",
            });
        }
        Command::StarknetBlockHashAndNumber => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_blockHashAndNumber",
                "params": [],
                "id": "1",
            });
        }
        Command::StarknetGetBlockTransactionCount { block_reference } => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_getBlockTransactionCount",
                "params": [block_reference],
                "id": "1",
            });
        }
        Command::StarknetGetBlockWithTxHashes { block_reference } => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_getBlockWithTxHashes",
                "params": [block_reference],
                "id": "1",
            });
        }
        Command::StarknetGetBlockWithTxs { block_reference } => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_getBlockWithTxs",
                "params": [block_reference],
                "id": "1",
            });
        }
        Command::StarknetGetClass {
            block_reference,
            class_hash,
        } => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_getClass",
                "params": [block_reference, class_hash],
                "id": "1",
            });
        }
        Command::StarknetGetClassAt {
            block_reference,
            address,
        } => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_getClassAt",
                "params": [block_reference, address],
                "id": "1",
            });
        }
        Command::StarknetGetClassHashAt {
            block_number,
            address,
        } => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_getClassHashAt",
                "params": [{
                    "block_number": block_number
                    },
                    address
                ],
                "id": "1",
            });
        }
        Command::StarknetEstimateFee(params) => {
            println!("{:#?}", params);
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_estimateFee",
                "params": {
                    "request": {
                        "type": params.request_type,
                        "max_fee": params.max_fee,
                        "version": params.version,
                        "nonce": params.nonce,
                        "signature": params.signature,
                        "sender_address": params.sender_address,
                        "calldata": params.calldata,
                    },
                    "block_id": params.block_id,
                },
                "id": "1",
            });
        }
        Command::StarknetGetEvents(get_events) => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_getEvents",
                "params": {
                    "filter": {
                        "from_block": get_events.from_block,
                        "to_block": get_events.to_block,
                        "keys": [get_events.keys],
                        "chunk_size": get_events.chunk_size,
                    }
                },
                "id": "1",
            });
        }
        Command::StarknetGetNonce {
            block_reference,
            address,
        } => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_getNonce",
                "params": [block_reference, address],
                "id": "1",
            });
        }
        Command::StarknetGetStateUpdate { block_reference } => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_getStateUpdate",
                "params": [block_reference],
                "id": "1",
            });
        }
        Command::StarknetGetStorageAt {
            address,
            key,
            block_reference,
        } => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_getStorageAt",
                "params": [address, key, block_reference],
                "id": "1",
            });
        }
        Command::StarknetCall {
            contract_address,
            entry_point_selector,
            calldata,
            block_reference,
        } => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_call",
                "params": [{
                    "contract_address": contract_address,
                    "entry_point_selector": entry_point_selector,
                    "calldata": calldata,
                    },
                    block_reference
                ],
                "id": "1",
            });
        }
    }

    handle_rpc_request(&rpc_url, &payload).await?;

    Ok(())
}

async fn handle_rpc_request(rpc_url: &str, payload: &Value) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let response: HashMap<String, Value> = client
        .post(rpc_url)
        .header("Content-Type", "application/json")
        .json(payload)
        .send()
        .await?
        .json()
        .await?;

    if response.contains_key("error") {
        println!("Error: \n {:#?}", response["error"]);
    } else {
        println!("Response: \n {:#?}", response["result"]);
    }

    Ok(())
}
