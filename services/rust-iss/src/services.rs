use crate::clients::HttpClient;
use crate::domain::IssTrend;
use crate::error::ApiError;
use crate::repo::{CacheRepo, IssRepo, OsdrRepo};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde_json::Value;
use sqlx::PgPool;

pub struct IssService {
    pool: PgPool,
    client: HttpClient,
}

impl IssService {
    pub fn new(pool: PgPool, client: HttpClient) -> Self {
        Self { pool, client }
    }

    pub async fn fetch_and_store(&self, url: &str) -> Result<(), ApiError> {
        let json = self.client.get_iss().await?;
        IssRepo::insert(&self.pool, url, json).await?;
        Ok(())
    }

    pub async fn get_last(&self) -> Result<Option<crate::domain::IssLog>, ApiError> {
        IssRepo::get_last(&self.pool).await
    }

    pub async fn get_trend(&self) -> Result<IssTrend, ApiError> {
        let rows = IssRepo::get_last_two(&self.pool).await?;

        if rows.len() < 2 {
            return Ok(IssTrend {
                movement: false,
                delta_km: 0.0,
                dt_sec: 0.0,
                velocity_kmh: None,
                from_time: None,
                to_time: None,
                from_lat: None,
                from_lon: None,
                to_lat: None,
                to_lon: None,
            });
        }

        let (t1, p1) = &rows[1];
        let (t2, p2) = &rows[0];

        let lat1 = num(&p1["latitude"]);
        let lon1 = num(&p1["longitude"]);
        let lat2 = num(&p2["latitude"]);
        let lon2 = num(&p2["longitude"]);
        let v2 = num(&p2["velocity"]);

        let mut delta_km = 0.0;
        let mut movement = false;
        if let (Some(a1), Some(o1), Some(a2), Some(o2)) = (lat1, lon1, lat2, lon2) {
            delta_km = haversine_km(a1, o1, a2, o2);
            movement = delta_km > 0.1;
        }
        let dt_sec = (*t2 - *t1).num_milliseconds() as f64 / 1000.0;

        Ok(IssTrend {
            movement,
            delta_km,
            dt_sec,
            velocity_kmh: v2,
            from_time: Some(*t1),
            to_time: Some(*t2),
            from_lat: lat1,
            from_lon: lon1,
            to_lat: lat2,
            to_lon: lon2,
        })
    }
}

fn num(v: &Value) -> Option<f64> {
    if let Some(x) = v.as_f64() {
        return Some(x);
    }
    if let Some(s) = v.as_str() {
        return s.parse::<f64>().ok();
    }
    None
}

fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let rlat1 = lat1.to_radians();
    let rlat2 = lat2.to_radians();
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2) + rlat1.cos() * rlat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    6371.0 * c
}

pub struct OsdrService {
    pool: PgPool,
    client: HttpClient,
}

impl OsdrService {
    pub fn new(pool: PgPool, client: HttpClient) -> Self {
        Self { pool, client }
    }

    pub async fn sync(&self) -> Result<(), ApiError> {
        let json = self.client.get_osdr().await?;
        
        let items = if let Some(a) = json.as_array() {
            a.clone()
        } else if let Some(v) = json.get("items").and_then(|x| x.as_array()) {
            v.clone()
        } else if let Some(v) = json.get("results").and_then(|x| x.as_array()) {
            v.clone()
        } else {
            vec![json.clone()]
        };

        for item in items {
            let id = s_pick(&item, &["dataset_id", "id", "uuid", "studyId", "accession", "osdr_id"]);
            let title = s_pick(&item, &["title", "name", "label"]);
            let status = s_pick(&item, &["status", "state", "lifecycle"]);
            let updated = t_pick(&item, &["updated", "updated_at", "modified", "lastUpdated", "timestamp"]);
            
            OsdrRepo::upsert(&self.pool, id, title, status, updated, item).await?;
        }
        Ok(())
    }

    pub async fn list(&self, limit: i64) -> Result<Vec<crate::domain::OsdrItem>, ApiError> {
        OsdrRepo::list(&self.pool, limit).await
    }
}

fn s_pick(v: &Value, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(x) = v.get(*k) {
            if let Some(s) = x.as_str() {
                if !s.is_empty() {
                    return Some(s.to_string());
                }
            } else if x.is_number() {
                return Some(x.to_string());
            }
        }
    }
    None
}

fn t_pick(v: &Value, keys: &[&str]) -> Option<DateTime<Utc>> {
    for k in keys {
        if let Some(x) = v.get(*k) {
            if let Some(s) = x.as_str() {
                if let Ok(dt) = s.parse::<DateTime<Utc>>() {
                    return Some(dt);
                }
                if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                    return Some(Utc.from_utc_datetime(&ndt));
                }
            } else if let Some(n) = x.as_i64() {
                return Some(Utc.timestamp_opt(n, 0).single().unwrap_or_else(Utc::now));
            }
        }
    }
    None
}

pub struct SpaceCacheService {
    pool: PgPool,
    client: HttpClient,
}

impl SpaceCacheService {
    pub fn new(pool: PgPool, client: HttpClient) -> Self {
        Self { pool, client }
    }

    pub async fn fetch_apod(&self) -> Result<(), ApiError> {
        let json = self.client.get_apod().await?;
        CacheRepo::insert(&self.pool, "apod", json).await?;
        Ok(())
    }

    pub async fn fetch_neo(&self) -> Result<(), ApiError> {
        let json = self.client.get_neo().await?;
        CacheRepo::insert(&self.pool, "neo", json).await?;
        Ok(())
    }

    pub async fn fetch_donki_flr(&self) -> Result<(), ApiError> {
        let json = self.client.get_donki_flr().await?;
        CacheRepo::insert(&self.pool, "flr", json).await?;
        Ok(())
    }

    pub async fn fetch_donki_cme(&self) -> Result<(), ApiError> {
        let json = self.client.get_donki_cme().await?;
        CacheRepo::insert(&self.pool, "cme", json).await?;
        Ok(())
    }

    pub async fn fetch_spacex(&self) -> Result<(), ApiError> {
        let json = self.client.get_spacex().await?;
        CacheRepo::insert(&self.pool, "spacex", json).await?;
        Ok(())
    }

    pub async fn get_latest(&self, source: &str) -> Result<Option<crate::domain::SpaceCache>, ApiError> {
        CacheRepo::get_latest(&self.pool, source).await
    }

    pub async fn get_summary(&self) -> Result<Value, ApiError> {
        let apod = self.get_latest("apod").await?
            .map(|c| serde_json::json!({"at": c.fetched_at, "payload": c.payload}))
            .unwrap_or(serde_json::json!({}));
        
        let neo = self.get_latest("neo").await?
            .map(|c| serde_json::json!({"at": c.fetched_at, "payload": c.payload}))
            .unwrap_or(serde_json::json!({}));
        
        let flr = self.get_latest("flr").await?
            .map(|c| serde_json::json!({"at": c.fetched_at, "payload": c.payload}))
            .unwrap_or(serde_json::json!({}));
        
        let cme = self.get_latest("cme").await?
            .map(|c| serde_json::json!({"at": c.fetched_at, "payload": c.payload}))
            .unwrap_or(serde_json::json!({}));
        
        let spacex = self.get_latest("spacex").await?
            .map(|c| serde_json::json!({"at": c.fetched_at, "payload": c.payload}))
            .unwrap_or(serde_json::json!({}));

        let iss_last = IssRepo::get_last(&self.pool).await?
            .map(|l| serde_json::json!({"at": l.fetched_at, "payload": l.payload}))
            .unwrap_or(serde_json::json!({}));

        let osdr_count = OsdrRepo::count(&self.pool).await?;

        Ok(serde_json::json!({
            "apod": apod,
            "neo": neo,
            "flr": flr,
            "cme": cme,
            "spacex": spacex,
            "iss": iss_last,
            "osdr_count": osdr_count
        }))
    }
}

