use serenity::{async_trait, client::{Context, EventHandler}, model::{channel::Message, prelude::Ready}};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Bot {} is ready", ready.user.name);
    }

    async fn message(&self, context: Context, message: Message) {
        
        

    }

}