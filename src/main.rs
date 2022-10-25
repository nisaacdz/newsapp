use std::error::Error;
use newsapi::{surfer, print_handler};
use dotenv::dotenv;

fn main() -> Result<(), Box<dyn Error>>{
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;
    let url = format!("https://newsapi.org/v2/top-headlines?country=us&apiKey={}", api_key);
    let articles = surfer::get_articles(&url)?;

    print_handler::render_articles(&articles);
    Ok(())
}