mod bot;
mod rest;
use std::borrow::Borrow;

use bot::bot::*;

#[tokio::main]
async fn main() -> bot::error::DynError<()> {
    
    let mut bot = Bot::new(
        std::env::args().nth(1).expect("No token has been defined").borrow()
    ).await?;

    match bot.start().await {
        Ok(_) => println!("Bot stopped successfully"),
        Err(why) => println!("Bot has been stopped with an error: {}", why)
    }

    Ok(())
}
