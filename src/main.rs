mod bot;
mod rest;
mod metric;

use std::{borrow::Borrow, io::{Write, stdin}, sync::Arc};
use tokio::sync::RwLock;
use bot::bot::*;

#[tokio::main]
async fn main() -> bot::error::DynError<()> {
    
    let bot = Arc::new(RwLock::new(Bot::new(
        std::env::args().nth(1).expect("No token has been defined").borrow()
    ).await?));

    let read = Arc::clone(&bot);
    tokio::spawn(async move {
        match read.write().await.start().await {
            Ok(_) => println!("Bot stopped successfully"),
            Err(why) => println!("Bot has been stopped with an error: {}", why)
        }
    });
 
    let mut input = String::new();
    loop {
        print!("> ");
        std::io::stdout().flush().expect("flush");

        std::io::stdin().read_line(&mut input).expect("input");

        if input == "metrics" {
            let guard = bot.read().await;
            let safebrowsing = guard.safebrowsing().await;
            let metrics = safebrowsing.metrics();

            println!("Metrics: {}", String::from_utf8(metrics.gather()).expect("metrics"));
        }
        
        input.clear();

    }

}
