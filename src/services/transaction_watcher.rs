use ethers::prelude::*;

pub async fn watch_transactions(url: &str) {
    let provider = Provider::<Http>::try_from(url).expect("Failed to connect to ether api");

    let block_number = provider.get_block_number().await.unwrap();
    println!("Current block number: {}", block_number);
}
