use crate::clients::HttpClient;
use crate::config::Config;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub http_client: HttpClient,
    pub config: Config,
}



