use std::io::Result;

use futures::stream::{self, StreamExt};
use linkify::LinkFinder;

pub struct Safebrowsing {
    denylist: Vec<String>,
    finder: LinkFinder
}

impl Safebrowsing {

    pub fn new() -> Self {
        Self {
            denylist: vec![],
            finder: LinkFinder::new()
        }
    }

    pub async fn is_safe(&mut self, input: &str) -> super::error::DynError<Option<()>> {
        let mut links = self.finder.links(input);

        if links.any(|link| self.denylist.contains(&link.as_str().to_lowercase())) {
            return Ok(Some(()));
        }
        
        let a = stream::iter(links).any(|link| {
            async move {
                match reqwest::get(&format!("{}{}", super::constants::SAFEBROWSING_ENDPOINT, link.as_str())).await {
                    Ok(response) => {
                        println!("{}", response.text().await.expect("text not found"));
                    },
                    Err(_) => {
                        println!("Was unable to get safebrowsing-response")
                    }
                }
                true
            }
        }).await;
        Ok(None)
    }

}