mod theme;

use dotenv::dotenv;
use newsapi::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;
    let url = format!("https://newsapi.org/v2/top-headlines?country=us&apiKey={}", api_key);
    let articles = get_articles(&url)?;

    render_articles(&articles);
    Ok(())
}

pub fn render_articles(articles: &Articles) {
    let mytheme = theme::default();
    for each_article in articles.articles.iter() {  
        mytheme.print_text(&format!("{}", each_article.title.as_str()));
        mytheme.print_text(&format!(">>{}\n\n", each_article.url.as_str()));
    }
}