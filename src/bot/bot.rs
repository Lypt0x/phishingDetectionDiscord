use super::{handler::Handler, error::DynError};
use serenity::{Client, client::ClientBuilder};

pub struct Bot {
    client: Client,
}

impl Bot {

    pub async fn start(&mut self) -> DynError<()> {
        self.client.start().await?;
        Ok(())
    }

    pub async fn new(token: &str) -> DynError<Self> {
        Ok(Self {
            client: ClientBuilder::new(token).event_handler(Handler::new()).await?
        })
    }

}