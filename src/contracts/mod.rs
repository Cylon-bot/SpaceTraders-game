use anyhow::anyhow;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{alloc, collections::HashMap, vec};

use crate::space_traders_client::SpaceTradersClient;
#[derive(Serialize, Deserialize, Debug)]
pub struct ContractDetails {
    #[serde(rename(deserialize = "id"))]
    contract_id: String,
    #[serde(rename(deserialize = "factionSymbol"))]
    faction: String,
    #[serde(rename(deserialize = "type"))]
    contract_type: String,
    #[serde(flatten)]
    deadline: String,
    #[serde(rename(deserialize = "onAccepted"), flatten)]
    payment_on_accepted: usize,
    #[serde(rename(deserialize = "onFulfilled"), flatten)]
    payment_on_fulfilled: usize,
    #[serde(rename(deserialize = "terms"))]
    deliver: DeliveryList,
    accepted: bool,
    fulfilled: bool,
    #[serde(rename(deserialize = "expiration"))]
    expiration_date: String,
    #[serde(rename(deserialize = "deadlineToAccept"))]
    deadline_to_accept: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DeliveryList {
    #[serde(rename(deserialize = "deliver"))]
    delivery: Vec<Delivery>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Delivery {
    #[serde(rename(deserialize = "tradeSymbol"))]
    ore_name: String,
    #[serde(rename(deserialize = "destinationSymbol"))]
    destination_delivery: String,
    #[serde(rename(deserialize = "unitsRequired"))]
    units_required: usize,
    #[serde(rename(deserialize = "unitsFulfilled"))]
    units_fulfilled: usize,
}

impl ContractDetails {
    async fn new(account_details: Value) -> Result<Self, anyhow::Error> {
        let deserialized: ContractDetails = serde_json::from_value(account_details)?;
        Ok(deserialized)
    }
}

pub async fn get_all_contracts(
    client_space_traders: &SpaceTradersClient,
) -> Result<ContractDetails, anyhow::Error> {
    let all_contracts_response = client_space_traders
        .get("https://api.spacetraders.io/v2/my/contracts")
        .await?;

    let mut all_contracts = Vec::new();
    let all_contracts_json_contents: Value = {
        let json_content: Result<Value, _> =
            serde_json::from_str(&all_contracts_response.text().await?);
        match json_content?.get("data") {
            Some(json_content) => Ok(json_content.clone()),
            None => Err(anyhow!("no data found")),
        }?
    };
    println!("{}", all_contracts_json_contents);
    if let serde_json::Value::Array(contracts) = all_contracts_json_contents {
        for contract in contracts {
            all_contracts.push(ContractDetails::new(contract).await?);
        }
    }
    println!("{:?}", all_contracts);
    // let account_details = ContractDetails::new(all_contracts_response).await?;
    // Ok(account_details)
    todo!()
}
