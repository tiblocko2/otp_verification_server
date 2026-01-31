use axum::{
    extract::State,
    routing::post,
    Router,
    Json,
};
use tokio::net::TcpListener;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::di::Container;
use crate::domain::otp::OtpStatus;

#[derive(Deserialize)]
pub struct RequestOtpDto {
    pub phone: String,
}

#[derive(Deserialize)]
pub struct VerifyOtpDto {
    pub phone: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
}

pub struct Server {
    port: u16,
    container: Arc<Container>,
}

impl Server {
    pub fn new(port: u16, container: Arc<Container>) -> Self {
        Self { port, container }
    }

    pub async fn run(self) {
        let app = get_router(self.container.clone());

        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))
            .await
            .unwrap();

        axum::serve(listener, app).await.unwrap();
    }
}

async fn request_otp_handler(
    State(container): State<Arc<Container>>,
    Json(payload): Json<RequestOtpDto>,
) -> Json<StatusResponse> {
    container.get_otp_query.execute(payload.phone).await;

    Json(StatusResponse {
        status: "code_generated".into(),
    })
}

async fn verify_otp_handler(
    State(container): State<Arc<Container>>,
    Json(payload): Json<VerifyOtpDto>,
) -> Json<StatusResponse> {
    let result = container
        .verify_otp_query
        .execute(payload.phone, payload.code)
        .await;

    let status = match result {
        OtpStatus::Verified => "verified",
        OtpStatus::InvalidCode => "invalid_code",
        OtpStatus::Expired => "expired",
    };

    Json(StatusResponse {
        status: status.into(),
    })
}

pub fn get_router(container: Arc<Container>) -> Router {
    Router::new()
        .route("/otp/request", post(request_otp_handler))
        .route("/otp/verify", post(verify_otp_handler))
        .with_state(container)
}
