use linkify::{Link, LinkFinder};

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

    pub async fn is_safe(&mut self, input: &str) -> bool {

        let links = self.finder.links(input).collect::<Vec<Link>>();
        for link in links {
            
            if self.denylist.contains(&link.as_str().to_string().to_lowercase()) {
                return true;
            }

            match reqwest::get(&format!("{}{}", super::constants::SAFEBROWSING_ENDPOINT, link.as_str())).await {
                Ok(response) => {
                    println!("{}", response.text().await.expect("response has no content"));
                },
                Err(_) => {
                    println!("Unable to get safebrowsing-response")
                }
            }
        }
        
        false
    }

}