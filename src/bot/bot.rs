use std::sync::Arc;

use tokio::sync::RwLock;
use crate::metric::Metrics;

use super::{handler::Handler, error::DynError};
use serenity::{Client, client::ClientBuilder};

pub struct Bot {
    client: Client,
    metrics: Arc<RwLock<Metrics>>
}

impl Bot {

    pub async fn start(&mut self) -> DynError<()> {
        self.client.start().await?;
        Ok(())
    }

    pub fn metrics(&self) -> &Arc<RwLock<Metrics>> {
        &self.metrics
    }

    pub async fn new(token: &str) -> DynError<Self> {
        let handler = Handler::new();
        let metrics = handler.metrics().await;
        Ok(Self {
            client: ClientBuilder::new(token).event_handler(handler).await?,
            metrics
        })
    }

}