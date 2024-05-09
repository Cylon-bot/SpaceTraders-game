use anyhow::anyhow;
use reqwest::{header, Client, Response};
use serde_yaml;
use std::{collections::BTreeMap, future::Future, io::BufReader};

struct SpaceTradersClient {
    client: Client,
    token: String,
}
impl SpaceTradersClient {
    fn new(client: Client, token: String) -> Self {
        SpaceTradersClient { client, token }
    }

    async fn get(&self, url: &str) -> Result<Response, anyhow::Error> {
        let a = self.client.get(url).bearer_auth(&self.token).send().await?;
        Ok(a)
    }
}

struct AccountDetails {
    account_id: String,
    account_name: String,
    headquarters: String,
    credit: i128,
    starting_faction: String,
    ship_count: u16,
}

impl AccountDetails {
    fn new(account_details_response: Response) -> Self {
        account_details_response.
        AccountDetails {}
    }
}
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let f = std::fs::File::open("etc/agents.yaml")?;
    let reader: BufReader<_> = BufReader::new(f);
    let agents: BTreeMap<String, String> = serde_yaml::from_reader(reader)?;
    let agent = match agents.get("CylonSpace") {
        Some(agent) => Ok(agent.clone()),
        None => Err(anyhow!("Agent 'CylonSpace' not found")),
    }?;
    let client_space_traders = SpaceTradersClient::new(Client::builder().build()?, agent);
    get_account_details(client_space_traders).await?;
    Ok(())
}

async fn get_account_details(
    client_space_traders: SpaceTradersClient,
) -> Result<AccountDetails, anyhow::Error> {
    let account_details_response = client_space_traders
        .get("https://api.spacetraders.io/v2/my/agent")
        .await?;
    let account_details = AccountDetails::new(account_details_response);
    Ok(account_details)
}
