mod bot;
mod rest;
mod metric;

use std::{borrow::Borrow, io::Write, sync::Arc};
use bot::bot::*;

#[tokio::main]
async fn main() -> bot::error::DynError<()> {
    
    let mut bot = Bot::new(
        std::env::args().nth(1).expect("No token has been defined").borrow()
    ).await?;

    let metrics = Arc::clone(bot.metrics());
    tokio::spawn(async move {
        let mut input = String::new();
        loop {
            print!("> ");
            std::io::stdout().flush().expect("flush");
    
            std::io::stdin().read_line(&mut input).expect("input");
    
            if input.trim() == "metrics" {
                let guard = metrics.read().await;
                println!("test: {}", String::from_utf8(guard.gather()).expect("metrics"));
            }
            
            input.clear();
    
        }
    });
 
    match bot.start().await {
        Ok(_) => println!("Bot stopped without any errors"),
        Err(why) => println!("Bot stopped with an error: {:?}", why)
    };
    Ok(())
}
