use starknet::core::crypto::Signature;
use starknet::core::types::FieldElement;
use starknet::signers::{LocalWallet, Signer, SigningKey, VerifyingKey};
use starknet_crypto::pedersen_hash;

async fn sign_message(
    private_key: &str,
    call_data: &Vec<String>,
) -> Result<Signature, <LocalWallet as Signer>::SignError> {
    let private_key = FieldElement::from_hex_be(private_key).unwrap();

    let signing_key = SigningKey::from_secret_scalar(private_key);
    let verifying_key = signing_key.verifying_key();

    let local_wallet = LocalWallet::from_signing_key(signing_key);

    let hash = compute_hash_on_elements(call_data);

    local_wallet.sign_hash(&hash).await
}

pub fn compute_hash_on_elements(vec: &Vec<String>) -> FieldElement {
    let fe_vec: Vec<FieldElement> = vec
        .iter()
        .map(|s| FieldElement::from_hex_be(s).unwrap())
        .collect();

    let felt_result = fe_vec
        .into_iter()
        .reduce(|x, y| pedersen_hash(&x, &y))
        .unwrap();

    felt_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sign_message() {
        let private_key = "0x00c1cf1490de1352865301bb8705143f3ef938f97fdf892f1090dcb5ac7bcd1d";
        let call_data = vec![
            "0x0000000000000000000000000000000000000000000000000000000000001111".to_string(),
            "0x36fa6de2810d05c3e1a0ebe23f60b9c2f4629bbead09e5a9704e1c5632630d5".to_string(),
            "0x0".to_string(),
        ];
        println!("=====");
        let signature = sign_message(private_key, &call_data).await;
        println!("===== {:?}", signature);
        // Add any assertions here if you want to check the result.
    }
}
