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

    #[structopt(short, long, use_delimiter = true)]
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

#[derive(StructOpt, Debug)]
pub struct AddInvokeTransactionOpts {
    #[structopt(long, default_value = "0xDEADB")]
    pub max_fee: String,

    #[structopt(long, default_value = "0x1")]
    pub version: String,

    #[structopt(long, default_value = "0x0")]
    pub nonce: String,

    #[structopt(long, use_delimiter = true)]
    pub signature: Vec<String>,

    #[structopt(long)]
    pub sender_address: String,

    #[structopt(long, use_delimiter = true)]
    pub calldata: Vec<String>,
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

    #[structopt(name = "get-nonce")]
    StarknetGetNonce {
        #[structopt(long, default_value = "latest")]
        block_reference: String,
        #[structopt(long)]
        address: String,
    },

    #[structopt(name = "get-state-update")]
    StarknetGetStateUpdate {
        #[structopt(long, default_value = "latest")]
        block_reference: String,
    },

    #[structopt(name = "get-storage-at")]
    StarknetGetStorageAt {
        #[structopt(long)]
        address: String,

        #[structopt(long)]
        key: String,

        #[structopt(long, default_value = "latest")]
        block_reference: String,
    },

    #[structopt(name = "get-tx-by-block-id-and-index")]
    StarknetGetTransactionByBlockIdAndIndex {
        #[structopt(long, default_value = "latest")]
        block_reference: String,

        #[structopt(long)]
        index: u32,
    },

    #[structopt(name = "get-tx-by-hash")]
    StarknetGetTransactionByHash {
        #[structopt(long)]
        tx_hash: String,
    },

    #[structopt(name = "get-tx-receipt")]
    StarknetGetTransactionReceipt {
        #[structopt(long)]
        tx_hash: String,
    },

    #[structopt(name = "get-pending-txs")]
    StarknetPendingTransactions,

    #[structopt(name = "get-syncing")]
    StarknetSyncing,

    #[structopt(name = "call-contract")]
    StarknetCall {
        #[structopt(long)]
        contract_address: String,

        #[structopt(long)]
        entry_point_selector: String,

        #[structopt(long, use_delimiter = true)]
        calldata: Vec<String>,

        #[structopt(long, default_value = "latest")]
        block_reference: String,
    },

    #[structopt(name = "invoke-tx")]
    StarknetAddInvokeTransaction(AddInvokeTransactionOpts),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "madara_cli", about = "CLI for Madara node.")]
pub struct Opt {
    #[structopt(long, default_value = "http://0.0.0.0:9944")]
    pub rpc_url: String,

    #[structopt(subcommand)]
    pub command: Command,
}
