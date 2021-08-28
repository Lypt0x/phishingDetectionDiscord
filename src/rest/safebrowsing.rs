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

    pub async fn is_safe(&mut self, input: &str) {

        /*
        -> Not using it yet because of debugging
        if links.any(|link| self.denylist.contains(&link.as_str().to_lowercase())) {
            return false;
        }*/

        let links = self.finder.links(input);

        for link in links {
            match reqwest::get(&format!("{}{}", super::constants::SAFEBROWSING_ENDPOINT, link.as_str())).await {
                Ok(response) => {
                    println!("{}", response.text().await.expect("response has no content"));
                },
                Err(_) => {
                    println!("Unable to get safebrowsing-response")
                }
            }
        }

    }

}