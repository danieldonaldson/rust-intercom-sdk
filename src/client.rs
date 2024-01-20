use reqwest::{header::HeaderMap, Response};
use serde::Serialize;

use crate::errors::Error;

#[derive(Debug, Clone)]
pub struct Client {
    http_client: reqwest::Client,
    access_token: String,
    endpoint: String,
    version: String,
}

impl Client {
    pub fn new(access_token: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            access_token,
            endpoint: "https://api.intercom.io/".to_string(),
            version: "2.10".to_string(),
        }
    }

    pub async fn get<T>(&self, url: String, params: &T) -> Result<Response, Error>
    where
        T: Serialize + ?Sized,
    {
        let url = format!("{}/{}", self.endpoint, url);
        let res = self
            .http_client
            .get(url.as_str())
            .headers(self.create_header()?)
            .query(&params)
            .send()
            .await?;
        Ok(res)
    }

    pub async fn post<T>(&self, url: String, params: &T) -> Result<Response, Error>
    where
        T: Serialize + ?Sized,
    {
        let url = format!("{}/{}", self.endpoint, url);
        let res = self
            .http_client
            .post(url.as_str())
            .headers(self.create_header()?)
            .json(&params)
            .send()
            .await?;
        Ok(res)
    }

    fn create_header(&self) -> Result<HeaderMap, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.access_token).parse()?,
        );
        headers.insert("Intercom-Version", self.version.parse()?);

        Ok(headers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        dotenv::dotenv().ok();
    }

    #[tokio::test]
    async fn create_client() {
        setup();
        let access_token = std::env::var("INTERCOM_ACCESS_TOKEN").unwrap();
        let client = Client::new(access_token);
        let res = client.get("contacts".to_string(), &vec![()]).await.unwrap();
        assert_eq!(res.status(), reqwest::StatusCode::OK);
    }
}
