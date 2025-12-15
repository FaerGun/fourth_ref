use axum::extract::{Path, Query, State};
use axum::Json;
use crate::app_state::AppState;
use crate::error::{ApiError, SuccessResponse};
use crate::services::{IssService, OsdrService, SpaceCacheService};
use chrono::Utc;
use serde_json::Value;
use std::collections::HashMap;

pub async fn health() -> Json<Value> {
    Json(serde_json::json!({
        "ok": true,
        "status": "ok",
        "now": Utc::now()
    }))
}

pub async fn last_iss(
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<Value>>, ApiError> {
    let service = IssService::new(state.pool.clone(), state.http_client.clone());
    let log = service.get_last().await?;
    
    if let Some(log) = log {
        Ok(Json(SuccessResponse::new(serde_json::json!({
            "id": log.id,
            "fetched_at": log.fetched_at,
            "source_url": log.source_url,
            "payload": log.payload
        }))))
    } else {
        Ok(Json(SuccessResponse::new(serde_json::json!({
            "message": "no data"
        }))))
    }
}

pub async fn trigger_iss(
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<Value>>, ApiError> {
    let service = IssService::new(state.pool.clone(), state.http_client.clone());
    service.fetch_and_store(&state.config.where_iss_url).await?;
    last_iss(State(state)).await
}

pub async fn iss_trend(
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<crate::domain::IssTrend>>, ApiError> {
    let service = IssService::new(state.pool.clone(), state.http_client.clone());
    let trend = service.get_trend().await?;
    Ok(Json(SuccessResponse::new(trend)))
}

pub async fn osdr_sync(
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<Value>>, ApiError> {
    let service = OsdrService::new(state.pool.clone(), state.http_client.clone());
    service.sync().await?;
    Ok(Json(SuccessResponse::new(serde_json::json!({
        "status": "ok"
    }))))
}

pub async fn osdr_list(
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<Value>>, ApiError> {
    let limit = std::env::var("OSDR_LIST_LIMIT")
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(20);
    
    let service = OsdrService::new(state.pool.clone(), state.http_client.clone());
    let items = service.list(limit).await?;
    
    let out: Vec<Value> = items.into_iter().map(|item| {
        serde_json::json!({
            "id": item.id,
            "dataset_id": item.dataset_id,
            "title": item.title,
            "status": item.status,
            "updated_at": item.updated_at,
            "inserted_at": item.inserted_at,
            "raw": item.raw,
        })
    }).collect();
    
    Ok(Json(SuccessResponse::new(serde_json::json!({
        "items": out
    }))))
}

pub async fn space_latest(
    Path(src): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<Value>>, ApiError> {
    let service = SpaceCacheService::new(state.pool.clone(), state.http_client.clone());
    let cache = service.get_latest(&src).await?;
    
    if let Some(cache) = cache {
        Ok(Json(SuccessResponse::new(serde_json::json!({
            "source": src,
            "fetched_at": cache.fetched_at,
            "payload": cache.payload
        }))))
    } else {
        Ok(Json(SuccessResponse::new(serde_json::json!({
            "source": src,
            "message": "no data"
        }))))
    }
}

pub async fn space_refresh(
    Query(q): Query<HashMap<String, String>>,
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<Value>>, ApiError> {
    let list = q.get("src")
        .cloned()
        .unwrap_or_else(|| "apod,neo,flr,cme,spacex".to_string());
    
    let service = SpaceCacheService::new(state.pool.clone(), state.http_client.clone());
    let mut done = Vec::new();
    
    for s in list.split(',').map(|x| x.trim().to_lowercase()) {
        match s.as_str() {
            "apod" => {
                let _ = service.fetch_apod().await;
                done.push("apod");
            }
            "neo" => {
                let _ = service.fetch_neo().await;
                done.push("neo");
            }
            "flr" => {
                let _ = service.fetch_donki_flr().await;
                done.push("flr");
            }
            "cme" => {
                let _ = service.fetch_donki_cme().await;
                done.push("cme");
            }
            "spacex" => {
                let _ = service.fetch_spacex().await;
                done.push("spacex");
            }
            _ => {}
        }
    }
    
    Ok(Json(SuccessResponse::new(serde_json::json!({
        "refreshed": done
    }))))
}

pub async fn space_summary(
    State(state): State<AppState>,
) -> Result<Json<SuccessResponse<Value>>, ApiError> {
    let service = SpaceCacheService::new(state.pool.clone(), state.http_client.clone());
    let summary = service.get_summary().await?;
    Ok(Json(SuccessResponse::new(summary)))
}

