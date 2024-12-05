use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
};

pub async fn redirect_to_youtube() -> impl IntoResponse {
    (
        StatusCode::FOUND,
        [(
            header::LOCATION,
            "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        )],
    )
}
