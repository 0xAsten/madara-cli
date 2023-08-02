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
        Command::StarknetBlockNumber => {
            payload = json!({
                "jsonrpc": "2.0",
                "method": "starknet_blockNumber",
                "params": [],
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
        println!("Error: {:?}", response["error"]);
    } else {
        println!("Response: {:?}", response["result"]);
    }

    Ok(())
}
