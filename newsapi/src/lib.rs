use serde::Deserialize;
use url::Url;

static BASE_URL: &str = "https://newsapi.org/v2";

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Could not fetch articles")]
    NewsApiRequestFailed(#[from] ureq::Error),
    #[error("Could not convert articles to string")]
    NewsApiConversionFailed(#[from] std::io::Error),
    #[error("Could not parse articles to Articles struct")]
    NewsApiParsingFailed(#[from] serde_json::Error),
    #[error("Could not parse given string into url")]
    NewsApiUrlParsingFailed(#[from] url::ParseError),
    #[error("Request failed {0}")]
    NewsApiBadRequestError(&'static str),
}

#[derive(Deserialize, Debug)]
pub struct Article {
    title: String,
    url: String,
}

impl Article{
    pub fn get_title(&self) -> &str{
        &self.title
    }

    pub fn get_url(&self) -> &str{
        &self.url
    }
}

pub enum EndPoint {
    HeadLines,
}

impl ToString for EndPoint {
    fn to_string(&self) -> String {
        match self {
            EndPoint::HeadLines => "top-headlines".to_string(),
        }
    }
}

pub enum Country {
    Us,
    Ghana,
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Country::Us => "us".to_string(),
            Country::Ghana => "china".to_string(),
        }
    }
}

pub struct NewsAPI {
    api_key: String,
    end_point: EndPoint,
    country: Country,
}

#[derive(Deserialize, Debug)]
pub struct NewsApiResponse {
    pub status: String,
    pub articles: Vec<Article>,
    pub code: Option<String>,
}

impl NewsAPI {
    pub fn new(api_key: String, end_point: EndPoint, country: Country) -> Self {
        NewsAPI {
            api_key: api_key,
            end_point: end_point,
            country: country,
        }
    }

    pub fn default(api_key: String) -> Self {
        Self::new(api_key, EndPoint::HeadLines, Country::Us)
    }

    pub fn set_end_point(&mut self, end_point: EndPoint) -> &mut Self {
        self.end_point = end_point;
        self
    }

    pub fn set_country(&mut self, country: Country) -> &mut Self {
        self.country = country;
        self
    }

    fn get_url(&self) -> Result<String, NewsApiError> {
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut()
            .unwrap()
            .push(&self.end_point.to_string());
        url.set_query(Some(&format!("country={}", self.country.to_string())));
        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<NewsApiResponse, NewsApiError> {
        let url = self.get_url().unwrap();

        let req = ureq::get(&url).set("Authorization", &self.api_key);

        let res: NewsApiResponse = req.call()?.into_json()?;

        match res.status.as_str() {
            "ok" => return Ok(res),
            _=> return Err(map_request_error(res.code)),
        };
    }
}

pub fn map_request_error(code: Option<String>) -> NewsApiError {
    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => {
                return NewsApiError::NewsApiBadRequestError("Api key disabled")
            }
            _ => return NewsApiError::NewsApiBadRequestError("Unknown Error"),
        }
    }

    NewsApiError::NewsApiBadRequestError("Unknown Error")
}
