use anyhow::Result;
use dotenv::dotenv;
use fuels::prelude::*;
use std::env;

mod indexer;

abigen!(Contract(
    name = "Greeter",
    abi = "contracts/greeter/out/debug/greeter-abi.json"
));

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    println!("Starting indexer...");

    let node_url = env::var("FUEL_NODE_URL").expect("FUEL_NODE_URL must be set in your .env file");
    let provider = Provider::connect(&node_url).await?;
    let contract_id: ContractId = env::var("GREETER_CONTRACT_ID")
        .expect("GREETER_CONTRACT_ID must be set")
        .parse()
        .unwrap();

    println!(
        "Indexer started. Listening for events from contract: {}",
        contract_id
    );

    indexer::run(provider, contract_id).await?;

    Ok(())
}
