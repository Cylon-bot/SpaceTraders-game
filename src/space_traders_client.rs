use reqwest::{Client, Response};

pub struct SpaceTradersClient {
    client: Client,
    token: String,
}
impl SpaceTradersClient {
    pub fn new(token: String) -> Result<Self, anyhow::Error> {
        Ok(SpaceTradersClient {
            client: Client::builder().build()?,
            token,
        })
    }

    pub async fn get(&self, url: &str) -> Result<Response, anyhow::Error> {
        Ok(self.client.get(url).bearer_auth(&self.token).send().await?)
    }
}
