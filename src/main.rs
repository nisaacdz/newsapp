use std::error::Error;
use serde::Deserialize;
use colour::{dark_green_ln, yellow_ln};

fn main() -> Result<(), Box<dyn Error>>{
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=ce262f1d2c1a4288a8960760763fc0b1";
    let articles = get_articles(url)?;

    render_articles(&articles);
    Ok(())
}

fn render_articles(articles: &Articles){
    for each_article in articles.articles.iter(){
        dark_green_ln!("> {}", each_article.title);
        yellow_ln!("> {}", each_article.url);
    }
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>>{
    let res = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&res)?;

    Ok(articles)
}

#[derive(Deserialize, Debug)]
struct Article{
    title: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Articles{
    articles: Vec<Article>,
}