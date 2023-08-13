use starknet::core::types::{BlockId, BlockTag, FieldElement};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use starknet::providers::Provider;

use url::Url;

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_read_from_contract() {
        let rpc_url = "http://localhost:9944";

        let contract_address = "0x0000000000000000000000000000000000000000000000000000000000000001";

        let rpc_client = JsonRpcClient::new(HttpTransport::new(Url::parse(rpc_url).unwrap()));
        let abi = rpc_client
            .get_class_at(
                BlockId::Tag(BlockTag::Latest),
                FieldElement::from_hex_be(contract_address).unwrap(),
            )
            .await;
        println!("{:#?}", abi);
    }
}
