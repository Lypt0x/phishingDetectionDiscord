pub mod bot;
mod handler;

pub mod error {
    pub type DynError<T> = Result<T, Box<dyn std::error::Error>>;
}