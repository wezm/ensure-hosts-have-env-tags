use reqwest::{self, Url};
use failure::Error;
use std::collections::HashMap;

lazy_static! {
    static ref API_URL: Url = "https://api.datadoghq.com/api/v1/".parse().unwrap();
}

#[derive(Debug, Deserialize)]
pub struct SearchResults {
    pub metrics: Vec<String>,
    pub hosts: Vec<String>,
}

#[derive(Deserialize)]
struct SearchResultsResponse {
    results: SearchResults,
}

#[derive(Deserialize)]
struct HostTagsResponse {
    tags: HashMap<String, Vec<String>>,
}

#[derive(Deserialize)]
struct SingleHostTagsResponse {
    tags: Vec<String>,
}

pub struct Client {
    http: reqwest::Client,
    api_key: String,
    app_key: String,
}

impl Client {
    pub fn new<S: Into<String>>(api_key: S, app_key: S) -> Self {
        Client {
            http: reqwest::Client::new(),
            api_key: api_key.into(),
            app_key: app_key.into(),
        }
    }

    fn get(&self, url: Url) -> reqwest::RequestBuilder {
        let mut builder = self.http.get(url);
        builder.query(&[("api_key", &self.api_key), ("application_key", &self.app_key)]);

        builder
    }

    pub fn search(&self, query: Option<&str>) -> Result<SearchResults, Error> {
        let url = API_URL.join("search")?;
        let resp: SearchResultsResponse = self.get(url).query(&[("q", query.unwrap_or(""))]).send()?.json()?;
        Ok(resp.results)
    }

    pub fn tags(&self) -> Result<HashMap<String, Vec<String>>, Error> {
        let url = API_URL.join("tags/hosts")?;
        let resp: HostTagsResponse = self.get(url).send()?.json()?;
        Ok(resp.tags)
    }

    pub fn host_tags(&self, host: &str) -> Result<Vec<String>, Error> {
        let url = API_URL.join("tags/hosts")?.join(host)?;
        let resp: SingleHostTagsResponse = self.get(url).send()?.json()?;
        Ok(resp.tags)
    }
}
