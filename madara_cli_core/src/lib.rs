use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "get-blocknumber")]
    StarknetBlockNumber,
    #[structopt(name = "call-contract")]
    StarknetCall {
        #[structopt(long)]
        contract_address: String,
        #[structopt(long)]
        entry_point_selector: String,
        #[structopt(long)]
        calldata: Vec<String>,
        #[structopt(default_value = "latest")]
        block_reference: String,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "madara_cli", about = "CLI for Madara node.")]
pub struct Opt {
    #[structopt(long, default_value = "http://0.0.0.0:9944")]
    pub rpc_url: String,

    #[structopt(subcommand)]
    pub command: Command,
}
