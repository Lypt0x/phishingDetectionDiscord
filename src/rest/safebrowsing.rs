use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use linkify::{Link, LinkFinder};
use serde_json::Value;
use url::Url;

use crate::metric::*;
pub struct Safebrowsing {
    denylist: HashMap<String, i64>,
    finder: LinkFinder,
    metrics: Arc<RwLock<Metrics>>
}

impl Safebrowsing {

    pub fn new() -> Self {
        Self {
            denylist: HashMap::new(),
            finder: LinkFinder::new(),
            metrics: Arc::new(RwLock::new(Metrics::new()))
        }
    }

    pub fn metrics(&self) -> Arc<RwLock<Metrics>> {
        Arc::clone(&self.metrics)
    }

    pub async fn is_safe(&mut self, input: &str) -> i64 {

        let links = self.finder.links(input).collect::<Vec<Link>>();
        for link in links {

            let mut url = Url::parse(&link.as_str()).expect("url");
            url.set_query(None);
            let url = url.to_string().to_ascii_lowercase();

            let mut metrics = self.metrics.write().await;

            if self.denylist.contains_key(&url) {
                metrics.increment(MetricType::Cached);
                return self.denylist[&url];
            }

            match reqwest::get(&format!("{}{}", super::constants::SAFEBROWSING_ENDPOINT, url)).await {
                Ok(response) => {
                    if let Ok(text) = response.text().await {
                        if let Ok(root) = serde_json::from_str::<Value>(&text[7..text.len()-1]) {
                            let safe_state = root.pointer("/1")
                                .expect("safe state pointee").as_u64()
                                .expect("safe state value");
                            if safe_state == 6 || safe_state == 1 || safe_state == 2 {
                                let time = root.pointer("/7")
                                    .expect("time pointee").as_i64().expect("time value");
                                self.denylist.insert(url, time);
                                metrics.increment(MetricType::Unhealthy);
                                return time;
                            }
                            metrics.increment(MetricType::Healthy);
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