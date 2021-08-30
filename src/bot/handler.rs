use std::sync::Arc;
use tokio::sync::Mutex;

use crate::rest::safebrowsing::Safebrowsing;

use serenity::{async_trait, client::{Context, EventHandler}, model::{channel::Message, prelude::Ready}};

pub struct Handler {
    safebrowsing: Arc<Mutex<Safebrowsing>>
}

impl Handler {
    pub fn new() -> Self {
        
        Self {
            safebrowsing: Arc::new(Mutex::new(Safebrowsing::new()))
        }
    }

}

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Bot {} is ready", ready.user.name);

    }

    async fn message(&self, _context: Context, message: Message) {
        self.safebrowsing.lock().await.is_safe(&message.content).await;
    }


}