use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;

pub async fn get_ddb_client() -> Client {
    let region = RegionProviderChain::default_provider();
    let config = aws_config::from_env().region(region).load().await;
    Client::new(&config)
}
