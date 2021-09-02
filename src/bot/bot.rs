use std::sync::Arc;

use tokio::sync::{RwLock, RwLockReadGuard};
use crate::{rest::safebrowsing::Safebrowsing};

use super::{handler::Handler, error::DynError};
use serenity::{Client, client::ClientBuilder};

pub struct Bot {
    client: Client,
    safebrowsing: Arc<RwLock<Safebrowsing>>
}

impl Bot {

    pub async fn start(&mut self) -> DynError<()> {
        self.client.start().await?;
        Ok(())
    }

    pub async fn safebrowsing(&self) -> RwLockReadGuard<'_, Safebrowsing> {
        self.safebrowsing.read().await
    }

    pub async fn new(token: &str) -> DynError<Self> {
        let handler = Handler::new();
        let safebrowsing = handler.safebrowsing().await;
        Ok(Self {
            client: ClientBuilder::new(token).event_handler(handler).await?,
            safebrowsing
        })
    }

}