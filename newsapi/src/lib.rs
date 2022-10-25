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

pub mod surfer {
    use crate::{Articles, NewsApiError};

    pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
        let res = ureq::get(url)
            .call()
            .map_err(|_| NewsApiError::NewsApiRequestFailed)?
            .into_string()
            .map_err(|_| NewsApiError::NewsApiConversionFailed)?;

        let articles: Articles =
            serde_json::from_str(&res).map_err(|_| NewsApiError::NewsApiParsingFailed)?;

        Ok(articles)
    }
}

pub mod print_handler {
    use crate::Articles;
    use colour::{dark_green_ln, yellow_ln};

    pub fn render_articles(articles: &Articles) {
        for each_article in articles.articles.iter() {
            dark_green_ln!("> {}", each_article.title.as_str());
            yellow_ln!(">> {}\n", each_article.url.as_str());
        }
    }

    /*
    fn color_print(obj: &str, color: &str){
        match color{
            "yellow" => yellow!("> {}\n", obj),
            "blue" => blue!("> {}\n", obj),
            "green" => green!("> {}\n", obj),
            "grey" => grey!("> {}\n", obj),
            "red" => red!("> {}\n", obj),
            "cyan" => cyan!("> {}\n", obj),
            "magenta" => magenta!("> {}\n", obj),
            "black" => black!("> {}\n", obj),
            "dark_yellow" => dark_yellow!("> {}\n", obj),
            "dark_blue" => dark_blue!("> {}\n", obj),
            "dark_green" => dark_green!("> {}\n", obj),
            "dark_grey" => dark_grey!("> {}\n", obj),
            "dark_red" => dark_red!("> {}\n", obj),
            "dark_cyan" => dark_cyan!("> {}\n", obj),
            "dark_magenta" => dark_magenta!("> {}\n", obj),
            _ => white!("> {}\n", obj),
        }
    }
    */
}
