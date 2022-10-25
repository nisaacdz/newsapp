use std::error::Error;

fn main() {
    let url = "https://newsapi.org/v2/top-headlines?country=us&apiKey=ce262f1d2c1a4288a8960760763fc0b1";
    let articles = get_articles(url);
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>>{
    let res = ureq::get(url).call()?.into_string()?;

    dbg!(res);

    todo!()
}

struct Article{
    title: String,
    url: String,
}

struct Articles{
    articles: Vec<Article>,
}