use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "madara_cli", about = "CLI for Madara node.")]
pub struct Opt {
    #[structopt(long, default_value = "http://0.0.0.0:9944")]
    pub rpc_url: String,

    #[structopt(short = "m", long = "method")]
    pub method: String,

    #[structopt(short = "p", long = "params")]
    pub params: Option<String>,
}
