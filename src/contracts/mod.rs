use crate::space_traders_client::SpaceTradersClient;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractDetails {
    contract_id: String,
    faction: String,
    contract_type: String,
    deadline: String,
    payment_on_accepted: usize,
    payment_on_fulfilled: usize,
    deliver_list: Vec<Delivery>,
    accepted: bool,
    fulfilled: bool,
    expiration_date: String,
    deadline_to_accept: String,
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
) -> Result<Vec<ContractDetails>, anyhow::Error> {
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
    if let serde_json::Value::Array(contracts) = all_contracts_json_contents {
        for contract in contracts {
            let jq_command: String = format!(
                "'{}' | jq '{{
                    contract_id: .id, 
                    faction: .factionSymbol, 
                    contract_type: .type, 
                    deadline: .terms.deadline, 
                    payment_on_accepted: .terms.payment.onAccepted, 
                    payment_on_fulfilled: .terms.payment.onFulfilled, 
                    deliver_list: .terms.deliver, 
                    accepted: .accepted, 
                    fulfilled: .fulfilled, 
                    expiration_date: .expiration, 
                    deadline_to_accept: .deadlineToAccept}}'",
                contract
            );
            let output_command = if cfg!(target_os = "windows") {
                Command::new("powershell")
                    .args(["/C", &jq_command])
                    .output()
                    .expect("failed to execute process")
            } else {
                Command::new("sh")
                    .args(["-c", &jq_command])
                    .output()
                    .expect("failed to execute process")
            };
            let output_to_json: Value =
                serde_json::from_str(&String::from_utf8(output_command.stdout)?)?;
            all_contracts.push(ContractDetails::new(output_to_json).await?);
        }
    }
    Ok(all_contracts)
}
