use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Could not fetch articles")]
    NewsApiRequestFailed,
    #[error("Could not convert articles to string")]
    NewsApiConversionFailed,
    #[error("Could not parse articles to Articles struct")]
    NewsApiParsingFailed,
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>,
}

pub fn get_articles(url: &str) -> Result<crate::Articles, NewsApiError> {
    let res = ureq::get(url)
        .call()
        .map_err(|_| NewsApiError::NewsApiRequestFailed)?
        .into_string()
        .map_err(|_| NewsApiError::NewsApiConversionFailed)?;

    let articles: crate::Articles =
        serde_json::from_str(&res).map_err(|_| NewsApiError::NewsApiParsingFailed)?;

    Ok(articles)
}
