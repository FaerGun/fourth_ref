mod app_state;
mod clients;
mod config;
mod domain;
mod error;
mod handlers;
mod repo;
mod routes;
mod services;

use app_state::AppState;
use config::Config;
use routes::create_router;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

async fn init_db(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    // ISS
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS iss_fetch_log(
            id BIGSERIAL PRIMARY KEY,
            fetched_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            source_url TEXT NOT NULL,
            payload JSONB NOT NULL
        )"
    )
    .execute(pool)
    .await?;

    // OSDR
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS osdr_items(
            id BIGSERIAL PRIMARY KEY,
            dataset_id TEXT,
            title TEXT,
            status TEXT,
            updated_at TIMESTAMPTZ,
            inserted_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            raw JSONB NOT NULL
        )"
    )
    .execute(pool)
    .await?;
    
    sqlx::query(
        "CREATE UNIQUE INDEX IF NOT EXISTS ux_osdr_dataset_id
         ON osdr_items(dataset_id) WHERE dataset_id IS NOT NULL"
    )
    .execute(pool)
    .await?;

    // универсальный кэш космоданных
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS space_cache(
            id BIGSERIAL PRIMARY KEY,
            source TEXT NOT NULL,
            fetched_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            payload JSONB NOT NULL
        )"
    )
    .execute(pool)
    .await?;
    
    sqlx::query("CREATE INDEX IF NOT EXISTS ix_space_cache_source ON space_cache(source,fetched_at DESC)")
        .execute(pool)
        .await?;

    Ok(())
}

async fn run_scheduler(state: AppState) {
    let intervals = state.config.fetch_intervals.clone();
    
    // OSDR scheduler с advisory lock
    {
        let st = state.clone();
        let osdr_interval = intervals.osdr;
        tokio::spawn(async move {
            loop {
                if let Err(e) = run_with_lock(&st, "osdr", || async {
                    let service = services::OsdrService::new(st.pool.clone(), st.http_client.clone());
                    service.sync().await.map_err(|e| anyhow::anyhow!("{}", e))
                }).await {
                    error!("osdr scheduler error: {:?}", e);
                }
                tokio::time::sleep(Duration::from_secs(osdr_interval)).await;
            }
        });
    }
    
    // ISS scheduler
    {
        let st = state.clone();
        let iss_interval = intervals.iss;
        tokio::spawn(async move {
            loop {
                if let Err(e) = run_with_lock(&st, "iss", || async {
                    let service = services::IssService::new(st.pool.clone(), st.http_client.clone());
                    service.fetch_and_store(&st.config.where_iss_url).await.map_err(|e| anyhow::anyhow!("{}", e))
                }).await {
                    error!("iss scheduler error: {:?}", e);
                }
                tokio::time::sleep(Duration::from_secs(iss_interval)).await;
            }
        });
    }
    
    // APOD scheduler
    {
        let st = state.clone();
        let apod_interval = intervals.apod;
        tokio::spawn(async move {
            loop {
                if let Err(e) = run_with_lock(&st, "apod", || async {
                    let service = services::SpaceCacheService::new(st.pool.clone(), st.http_client.clone());
                    service.fetch_apod().await.map_err(|e| anyhow::anyhow!("{}", e))
                }).await {
                    error!("apod scheduler error: {:?}", e);
                }
                tokio::time::sleep(Duration::from_secs(apod_interval)).await;
            }
        });
    }
    
    // NeoWs scheduler
    {
        let st = state.clone();
        let neo_interval = intervals.neo;
        tokio::spawn(async move {
            loop {
                if let Err(e) = run_with_lock(&st, "neo", || async {
                    let service = services::SpaceCacheService::new(st.pool.clone(), st.http_client.clone());
                    service.fetch_neo().await.map_err(|e| anyhow::anyhow!("{}", e))
                }).await {
                    error!("neo scheduler error: {:?}", e);
                }
                tokio::time::sleep(Duration::from_secs(neo_interval)).await;
            }
        });
    }
    
    // DONKI scheduler
    {
        let st = state.clone();
        let donki_interval = intervals.donki;
        tokio::spawn(async move {
            loop {
                if let Err(e) = run_with_lock(&st, "donki", || async {
                    let service = services::SpaceCacheService::new(st.pool.clone(), st.http_client.clone());
                    service.fetch_donki_flr().await.map_err(|e| anyhow::anyhow!("{}", e))?;
                    service.fetch_donki_cme().await.map_err(|e| anyhow::anyhow!("{}", e))?;
                    Ok(())
                }).await {
                    error!("donki scheduler error: {:?}", e);
                }
                tokio::time::sleep(Duration::from_secs(donki_interval)).await;
            }
        });
    }
    
    // SpaceX scheduler
    {
        let st = state.clone();
        let spacex_interval = intervals.spacex;
        tokio::spawn(async move {
            loop {
                if let Err(e) = run_with_lock(&st, "spacex", || async {
                    let service = services::SpaceCacheService::new(st.pool.clone(), st.http_client.clone());
                    service.fetch_spacex().await.map_err(|e| anyhow::anyhow!("{}", e))
                }).await {
                    error!("spacex scheduler error: {:?}", e);
                }
                tokio::time::sleep(Duration::from_secs(spacex_interval)).await;
            }
        });
    }
}

async fn run_with_lock<F, Fut>(
    state: &AppState,
    lock_name: &str,
    task: F,
) -> anyhow::Result<()>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<()>>,
{
    // Используем PostgreSQL advisory lock для защиты от наложения задач
    let lock_id = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        lock_name.hash(&mut hasher);
        hasher.finish() as i64
    };
    
    // Пытаемся получить блокировку (неблокирующий режим)
    let lock_acquired: Option<bool> = sqlx::query_scalar(
        "SELECT pg_try_advisory_lock($1)"
    )
    .bind(lock_id)
    .fetch_optional(&state.pool)
    .await?;
    
    if lock_acquired.unwrap_or(false) {
        let result = task().await;
        // Освобождаем блокировку
        let _ = sqlx::query("SELECT pg_advisory_unlock($1)")
            .bind(lock_id)
            .execute(&state.pool)
            .await;
        result
    } else {
        info!("Skipping {} task - already running", lock_name);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    dotenvy::dotenv().ok();

    let config = Config::from_env()?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;
    
    init_db(&pool).await?;

    let http_client = clients::HttpClient::new(config.clone())?;
    
    let state = AppState {
        pool: pool.clone(),
        http_client,
        config: config.clone(),
    };

    // Запускаем фоновые задачи
    run_scheduler(state.clone());

    let app = create_router().with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3000)).await?;
    info!("rust_iss listening on 0.0.0.0:3000");
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
