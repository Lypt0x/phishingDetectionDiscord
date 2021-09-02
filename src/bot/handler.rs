use std::sync::{Arc};
use tokio::sync::{RwLock};

use crate::{metric::Metrics, rest::safebrowsing::{Safebrowsing}};

use serenity::{async_trait, client::{Context, EventHandler}, model::{channel::Message, prelude::Ready}};

pub struct Handler {
    safebrowsing: Arc<RwLock<Safebrowsing>>,
}

impl Handler {
   
    pub fn new() -> Self {
        let safebrowsing = Arc::new(RwLock::new(Safebrowsing::new()));
        Self {
            safebrowsing
        }
    }

    pub async fn metrics(&self) -> Arc<RwLock<Metrics>> {
        self.safebrowsing.read().await.metrics()
    }

}

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Bot {} is ready", ready.user.name);
    }

    async fn message(&self, context: Context, message: Message) {
        let safe = self.safebrowsing.write().await.is_safe(&message.content).await;
        if safe != -1 {
            message.delete(Arc::clone(&context.http))
                .await.expect(&format!("phising @ {}", message.id.0));
        }
    }

}