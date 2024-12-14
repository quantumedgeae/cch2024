use std::{sync::Arc, time::Duration};

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};

use ratelimit::Ratelimiter;
use tokio::time::sleep;

mod cch;

pub async fn hello_bird() -> &'static str {
    "Hello, bird!"
}

const REFILLS: u64 = 6;

struct AppState {
    rate_limiter: Ratelimiter,
}

async fn limit_rate(
    Extension(state): Extension<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, ()> {
    let state_rate_limiter = &state.rate_limiter;
    if let Err(sleep_duration) = state_rate_limiter.try_wait() {
        let _ = sleep(sleep_duration);
    };
    // println!("M->{}", state_rate_limiter.available());
    if state_rate_limiter.available() != 0 {
        Ok(next.run(request).await)
    } else {
        Ok((StatusCode::BAD_REQUEST, "No milk available\n".to_owned()).into_response())
    }
}
async fn refill(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let state_rate_limiter = &state.rate_limiter;
    let _ = state_rate_limiter.set_available(REFILLS);
    // println!("R->{}", state_rate_limiter.available());
    StatusCode::OK
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let rate_limiter = Ratelimiter::builder(1, Duration::from_millis(1000))
        .initial_available(REFILLS)
        .max_tokens(REFILLS)
        .build()
        .unwrap();
    let shared_state = Arc::new(AppState { rate_limiter });
    let router = Router::new()
        .route("/", get(hello_bird))
        .route("/-1/seek", get(cch::_minus1::redirect_to_youtube))
        .route("/2/dest", get(cch::challenge2::calc_ip_ops))
        .route("/2/key", get(cch::challenge2::calc_ip_ops))
        .route("/2/v6/dest", get(cch::challenge2::calc_ip_ops))
        .route("/2/v6/key", get(cch::challenge2::calc_ip_ops))
        .route("/5/manifest", post(cch::challenge5::manifest_messaging))
        .route("/9/milk", post(cch::challenge9::milk))
        .layer(middleware::from_fn(limit_rate))
        .route("/9/refill", post(refill))
        .layer(Extension(shared_state));

    Ok(router.into())
}
