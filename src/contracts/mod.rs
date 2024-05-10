use reqwest::Response;
use serde::{Deserialize, Serialize};

use crate::space_traders_client::SpaceTradersClient;
#[derive(Serialize, Deserialize, Debug)]
pub struct ContractDetails {
    #[serde(rename(deserialize = "accountId"))]
    account_id: String,
    #[serde(rename(deserialize = "symbol"))]
    account_name: String,
    headquarters: String,
    credits: i128,
    #[serde(rename(deserialize = "startingFaction"))]
    starting_faction: String,
    #[serde(rename(deserialize = "shipCount"))]
    ship_count: u16,
}

impl ContractDetails {
    async fn new(account_details_response: Response) {}
}

pub async fn get_all_contracts(
    client_space_traders: &SpaceTradersClient,
) -> Result<ContractDetails, anyhow::Error> {
    let all_contracts_response = client_space_traders
        .get("https://api.spacetraders.io/v2/my/contracts")
        .await?;

    println!("{:?}", all_contracts_response.text().await?);
    // let account_details = ContractDetails::new(all_contracts_response).await?;
    // Ok(account_details)
    todo!()
}
