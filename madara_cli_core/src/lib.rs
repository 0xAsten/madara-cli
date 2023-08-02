use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct StarknetEstimateFee {
    #[structopt(short, long, default_value = "INVOKE")]
    pub request_type: String,

    #[structopt(short, long, default_value = "0x0")]
    pub max_fee: String,

    #[structopt(short, long, default_value = "0x1")]
    pub version: String,

    #[structopt(short, long, default_value = "0x0")]
    pub nonce: String,

    #[structopt(short, long)]
    pub signature: Vec<String>,

    #[structopt(
        long,
        default_value = "0x0000000000000000000000000000000000000000000000000000000000000001"
    )]
    pub sender_address: String,

    #[structopt(short, long, use_delimiter = true)]
    pub calldata: Vec<String>,

    #[structopt(short, long, default_value = "latest")]
    pub block_id: String,
}

#[derive(Debug, StructOpt)]
pub struct GetEvents {
    #[structopt(long, default_value = "latest")]
    pub from_block: String,

    #[structopt(long, default_value = "latest")]
    pub to_block: String,

    #[structopt(long, use_delimiter = true)]
    pub keys: Vec<String>,

    #[structopt(long, default_value = "10")]
    pub chunk_size: u64,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "get-chainid")]
    StarknetChainId,

    #[structopt(name = "get-blocknumber")]
    StarknetBlockNumber,

    #[structopt(name = "get-block-hash-and-number")]
    StarknetBlockHashAndNumber,

    #[structopt(name = "get-block-txs-coount")]
    StarknetGetBlockTransactionCount {
        #[structopt(long, default_value = "latest")]
        block_reference: String,
    },

    #[structopt(name = "get-block-with-tx-hashes")]
    StarknetGetBlockWithTxHashes {
        #[structopt(long, default_value = "latest")]
        block_reference: String,
    },

    #[structopt(name = "get-block-with-txs")]
    StarknetGetBlockWithTxs {
        #[structopt(long, default_value = "latest")]
        block_reference: String,
    },

    #[structopt(name = "get-class")]
    StarknetGetClass {
        #[structopt(long, default_value = "latest")]
        block_reference: String,
        #[structopt(long)]
        class_hash: String,
    },

    #[structopt(name = "get-class-at")]
    StarknetGetClassAt {
        #[structopt(long, default_value = "latest")]
        block_reference: String,
        #[structopt(long)]
        address: String,
    },

    #[structopt(name = "get-class-hash-at")]
    StarknetGetClassHashAt {
        #[structopt(long)]
        block_number: u32,
        #[structopt(long)]
        address: String,
    },

    // TODO: Erro: "message": String("invalid type: map, expected a sequence at line 1 column 31")
    #[structopt(name = "get-estimate-fee")]
    StarknetEstimateFee(StarknetEstimateFee),

    #[structopt(name = "get-events")]
    StarknetGetEvents(GetEvents),

    #[structopt(name = "call-contract")]
    StarknetCall {
        #[structopt(long)]
        contract_address: String,
        #[structopt(long)]
        entry_point_selector: String,
        #[structopt(long)]
        calldata: Vec<String>,
        #[structopt(long, default_value = "latest")]
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
