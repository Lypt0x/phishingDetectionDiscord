mod bot;
use bot::bot::*;

#[tokio::main]
async fn main() {
    
    let bot = Bot::new("").await;

}
