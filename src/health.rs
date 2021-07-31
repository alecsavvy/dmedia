use crate::error::Result;
use actix_web::{get, web::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HealthCheck {
    redis: bool,
    chain: bool,
}

#[get("/health")]
pub async fn health_check() -> Result<Json<HealthCheck>> {
    Ok(Json(HealthCheck {
        redis: true,
        chain: true,
    }))
}
