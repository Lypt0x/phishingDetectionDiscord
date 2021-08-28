pub mod safebrowsing;

pub mod constants {
    pub static SAFEBROWSING_ENDPOINT: &'static str = "https://transparencyreport.google.com/transparencyreport/api/v3/safebrowsing/status?site=";
}

pub mod error {
    pub type DynError<T> = Result<T, Box<dyn std::error::Error>>;
}