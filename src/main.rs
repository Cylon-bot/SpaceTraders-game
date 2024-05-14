mod agents;
mod contracts;
mod space_traders_client;

use anyhow::anyhow;
use std::{collections::BTreeMap, io::BufReader};

use crate::agents::get_account_details;
use crate::contracts::get_all_contracts;
use crate::space_traders_client::SpaceTradersClient;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let f = std::fs::File::open("etc/agents.yaml")?;
    let reader: BufReader<_> = BufReader::new(f);
    let agents: BTreeMap<String, String> = serde_yaml::from_reader(reader)?;
    let agent = match agents.get("CylonSpace") {
        Some(agent) => Ok(agent.clone()),
        None => Err(anyhow!("Agent 'CylonSpace' not found")),
    }?;
    let client_space_traders = SpaceTradersClient::new(agent)?;
    let account_details = get_account_details(&client_space_traders).await?;
    let contracts_details = get_all_contracts(&client_space_traders).await?;
    println!("{:?}", contracts_details);
    Ok(())
}
