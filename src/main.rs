use axum::{
    routing::{get, post},
    Router,
};

mod cch;

pub async fn hello_bird() -> &'static str {
    "Hello, bird!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_bird))
        .route("/-1/seek", get(cch::_minus1::redirect_to_youtube))
        .route("/2/dest", get(cch::challenge2::calc_ip_ops))
        .route("/2/key", get(cch::challenge2::calc_ip_ops))
        .route("/2/v6/dest", get(cch::challenge2::calc_ip_ops))
        .route("/2/v6/key", get(cch::challenge2::calc_ip_ops))
        .route("/5/manifest", post(cch::challenge5::manifest_messaging));

    Ok(router.into())
}
