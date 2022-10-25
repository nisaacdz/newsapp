use std::error::Error;
use news_api::{surfer, print_handler};

fn main() -> Result<(), Box<dyn Error>>{
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=ce262f1d2c1a4288a8960760763fc0b1";
    let articles = surfer::get_articles(url)?;

    print_handler::render_articles(&articles);
    Ok(())
}