use std::collections::HashMap;

use linkify::{Link, LinkFinder};
use serde_json::Value;
use url::Url;

pub struct Safebrowsing {
    denylist: HashMap<String, i64>,
    finder: LinkFinder
}

impl Safebrowsing {

    pub fn new() -> Self {
        Self {
            denylist: HashMap::new(),
            finder: LinkFinder::new()
        }
    }

    pub async fn is_safe(&mut self, input: &str) -> i64 {

        let links = self.finder.links(input).collect::<Vec<Link>>();
        for link in links {

            let mut url = Url::parse(&link.as_str()).expect("url");
            url.set_query(None);
            let url = url.to_string().to_ascii_lowercase();

            if self.denylist.contains_key(&url) {
                return self.denylist[&url];
            }

            match reqwest::get(&format!("{}{}", super::constants::SAFEBROWSING_ENDPOINT, url)).await {
                Ok(response) => {
                    if let Ok(text) = response.text().await {
                        if let Ok(root) = serde_json::from_str::<Value>(&text[7..text.len()-1]) {
                            let safe_state = root.pointer("/1")
                                .expect("safe state pointee").as_u64()
                                .expect("safe state value");
                            if safe_state == 6 || safe_state == 1 {
                                let time = root.pointer("/7").expect("time pointee").as_i64().expect("time value");
                                self.denylist.insert(url, time);
                                return time;
                            }
                        }
                    }   
                },
                Err(_) => {
                    println!("Unable to get safebrowsing-response @ {}", url)
                }
            }
        }
        
        -1
    }

}