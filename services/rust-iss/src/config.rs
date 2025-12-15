use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub nasa_api_url: String,
    pub nasa_api_key: String,
    pub where_iss_url: String,
    pub fetch_intervals: FetchIntervals,
    pub http_timeout_secs: u64,
    pub max_retries: u32,
    pub retry_delay_secs: u64,
}

#[derive(Clone, Debug)]
pub struct FetchIntervals {
    pub osdr: u64,
    pub iss: u64,
    pub apod: u64,
    pub neo: u64,
    pub donki: u64,
    pub spacex: u64,
}

impl FetchIntervals {
    pub fn new() -> Self {
        Self {
            osdr: 600,
            iss: 120,
            apod: 43200,
            neo: 7200,
            donki: 3600,
            spacex: 3600,
        }
    }
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL is required");
        
        let nasa_api_url = std::env::var("NASA_API_URL")
            .unwrap_or_else(|_| "https://visualization.osdr.nasa.gov/biodata/api/v2/datasets/?format=json".to_string());
        
        let nasa_api_key = std::env::var("NASA_API_KEY").unwrap_or_default();
        
        let where_iss_url = std::env::var("WHERE_ISS_URL")
            .unwrap_or_else(|_| "https://api.wheretheiss.at/v1/satellites/25544".to_string());

        Ok(Config {
            database_url,
            nasa_api_url,
            nasa_api_key,
            where_iss_url,
            fetch_intervals: FetchIntervals {
                osdr: env_u64("FETCH_EVERY_SECONDS", 600),
                iss: env_u64("ISS_EVERY_SECONDS", 120),
                apod: env_u64("APOD_EVERY_SECONDS", 43200),
                neo: env_u64("NEO_EVERY_SECONDS", 7200),
                donki: env_u64("DONKI_EVERY_SECONDS", 3600),
                spacex: env_u64("SPACEX_EVERY_SECONDS", 3600),
            },
            http_timeout_secs: env_u64("HTTP_TIMEOUT_SECS", 30),
            max_retries: env_u32("MAX_RETRIES", 3),
            retry_delay_secs: env_u64("RETRY_DELAY_SECS", 2),
        })
    }
}

fn env_u64(k: &str, d: u64) -> u64 {
    std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(d)
}

fn env_u32(k: &str, d: u32) -> u32 {
    std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(d)
}


