use std::{collections::HashMap, vec};

use reqwest::Error;
use serde_json::{json, Value};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "madara_cli", about = "CLI for Madara node.")]
struct Opt {
    #[structopt(long, default_value = "http://0.0.0.0:9944")]
    rpc_url: String,

    #[structopt(short = "m", long = "method")]
    method: String,

    #[structopt(short = "p", long = "params")]
    params: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let rpc_url = opt.rpc_url;

    let payload = json!({
        "jsonrpc": "2.0",
        "method": opt.method,
        "params": serde_json::from_str::<Vec<String>>(&opt.params.unwrap_or("[]".to_string())).unwrap_or(vec![]),
        "id": "1",
    });

    let client = reqwest::Client::new();
    let response: HashMap<String, Value> = client
        .post(rpc_url)
        .header("Content-Type", "application/json")
        .json(&payload)
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
