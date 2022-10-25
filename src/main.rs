mod theme;

use dotenv::dotenv;
use newsapi::{NewsAPI, Article};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    dotenv()?;
    let api_key = std::env::var("API_KEY")?;

    let result: &Vec<Article> = &NewsAPI::default(api_key).fetch()?.articles;

    render_articles(result);
    Ok(())
}

pub fn render_articles(articles: &Vec<Article>) {
    let mytheme = theme::default();
    mytheme.print_text("# Top Headlines\n\n");
    for each_article in articles.iter() { 
        mytheme.print_text("---"); 
        mytheme.print_text(&format!("`{}`", each_article.get_title()));
        mytheme.print_text(&format!(">*{}*", each_article.get_url()));
    }
}