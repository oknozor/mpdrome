use std::sync::Arc;

use mpdrome_bridge::MpdBridge;
use submarine::{Client, auth::AuthBuilder};

#[derive(Clone)]
pub struct SubSonicClient {
    client: Arc<Client>,
    rt: Arc<tokio::runtime::Runtime>,
}

impl SubSonicClient {
    pub fn new(user: &str, password: &str, url: &str) -> Self {
        let client_name = format!("mpdrome-{user}");
        let auth = AuthBuilder::new(user, "v0.16.1")
            .client_name(&client_name)
            .hashed(password);
        let client = Client::new(url, auth);
        let rt = tokio::runtime::Runtime::new().unwrap();

        Self {
            client: Arc::new(client),
            rt: Arc::new(rt),
        }
    }
}

impl MpdBridge for SubSonicClient {
    fn list_artist(&self) -> color_eyre::Result<Vec<String>> {
        let response = self.rt.block_on(self.client.get_artists(None))?;

        response
            .into_iter()
            .flat_map(|indexi3| indexi3.artist.into_iter().map(|artist| Ok(artist.name)))
            .collect()
    }

    fn get_artist(&self) -> color_eyre::Result<mpdrome_bridge::Artist> {
        todo!()
    }
}
