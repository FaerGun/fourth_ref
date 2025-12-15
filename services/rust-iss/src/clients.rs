use crate::config::Config;
use crate::error::ApiError;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    config: Config,
}

impl HttpClient {
    pub fn new(config: Config) -> Result<Self, ApiError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.http_timeout_secs))
            .user_agent("rust_iss/1.0")
            .build()?;
        
        Ok(Self { client, config })
    }

    pub async fn get_with_retry(&self, url: &str) -> Result<Value, ApiError> {
        let mut last_error = None;
        
        for attempt in 0..=self.config.max_retries {
            match self.client.get(url).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        return Ok(resp.json().await?);
                    } else if attempt < self.config.max_retries {
                        tokio::time::sleep(Duration::from_secs(self.config.retry_delay_secs)).await;
                        continue;
                    } else {
                        // Если статус не успешный, используем последнюю ошибку или создаем новую
                        // Просто продолжаем к последней ошибке
                    }
                }
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.config.max_retries {
                        tokio::time::sleep(Duration::from_secs(self.config.retry_delay_secs)).await;
                    }
                }
            }
        }
        
        Err(ApiError::Http(last_error.unwrap()))
    }

    pub async fn get_iss(&self) -> Result<Value, ApiError> {
        self.get_with_retry(&self.config.where_iss_url).await
    }

    pub async fn get_osdr(&self) -> Result<Value, ApiError> {
        let mut url = self.config.nasa_api_url.clone();
        if !self.config.nasa_api_key.is_empty() {
            url = format!("{}&api_key={}", url, self.config.nasa_api_key);
        }
        self.get_with_retry(&url).await
    }

    pub async fn get_apod(&self) -> Result<Value, ApiError> {
        let mut url = "https://api.nasa.gov/planetary/apod?thumbs=true".to_string();
        if !self.config.nasa_api_key.is_empty() {
            url = format!("{}&api_key={}", url, self.config.nasa_api_key);
        }
        self.get_with_retry(&url).await
    }

    pub async fn get_neo(&self) -> Result<Value, ApiError> {
        let today = chrono::Utc::now().date_naive();
        let start = today - chrono::Days::new(2);
        let mut url = format!(
            "https://api.nasa.gov/neo/rest/v1/feed?start_date={}&end_date={}",
            start, today
        );
        if !self.config.nasa_api_key.is_empty() {
            url = format!("{}&api_key={}", url, self.config.nasa_api_key);
        }
        self.get_with_retry(&url).await
    }

    pub async fn get_donki_flr(&self) -> Result<Value, ApiError> {
        let (from, to) = last_days(5);
        let mut url = format!(
            "https://api.nasa.gov/DONKI/FLR?startDate={}&endDate={}",
            from, to
        );
        if !self.config.nasa_api_key.is_empty() {
            url = format!("{}&api_key={}", url, self.config.nasa_api_key);
        }
        self.get_with_retry(&url).await
    }

    pub async fn get_donki_cme(&self) -> Result<Value, ApiError> {
        let (from, to) = last_days(5);
        let mut url = format!(
            "https://api.nasa.gov/DONKI/CME?startDate={}&endDate={}",
            from, to
        );
        if !self.config.nasa_api_key.is_empty() {
            url = format!("{}&api_key={}", url, self.config.nasa_api_key);
        }
        self.get_with_retry(&url).await
    }

    pub async fn get_spacex(&self) -> Result<Value, ApiError> {
        self.get_with_retry("https://api.spacexdata.com/v4/launches/next").await
    }
}

fn last_days(n: i64) -> (String, String) {
    let to = chrono::Utc::now().date_naive();
    let from = to - chrono::Days::new(n as u64);
    (from.to_string(), to.to_string())
}

