use crate::Greeter;
use anyhow::Result;
use fuels::accounts::signers::private_key::PrivateKeySigner;
use fuels::accounts::wallet::Wallet;
use fuels::crypto::SecretKey;
use fuels::prelude::*;
use fuels::types::ContractId;
use std::str::FromStr;
use std::time::Duration;

pub async fn run(provider: Provider, contract_id: ContractId) -> Result<()> {
    let private_key_str =
        std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in the .env");
    let secret_key = SecretKey::from_str(&private_key_str).unwrap();
    let signer = PrivateKeySigner::new(secret_key);
    let wallet = Wallet::new(signer, provider.clone());
    let contract_instance = Greeter::new(contract_id, wallet.clone());

    loop {
        let response = contract_instance.methods().get_value().call().await?;

        println!("Contracts current value: {}", response.value);

        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}
