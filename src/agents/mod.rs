use crate::SpaceTradersClient;
use anyhow::anyhow;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountDetails {
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

impl AccountDetails {
    async fn new(account_details_response: Response) -> Result<Self, anyhow::Error> {
        let json_content: Value = {
            let json_content: Result<Value, _> =
                serde_json::from_str(&account_details_response.text().await?);
            match json_content?.get("data") {
                Some(json_content) => Ok(json_content.clone()),
                None => Err(anyhow!("no data found")),
            }?
        };
        let deserialized: AccountDetails = serde_json::from_value(json_content)?;
        Ok(deserialized)
    }
}

pub async fn get_account_details(
    client_space_traders: SpaceTradersClient,
) -> Result<AccountDetails, anyhow::Error> {
    let account_details_response = client_space_traders
        .get("https://api.spacetraders.io/v2/my/agent")
        .await?;
    let account_details = AccountDetails::new(account_details_response).await?;
    Ok(account_details)
}
