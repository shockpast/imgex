use std::env;

use axum::{
    body::Body,
    extract::Multipart,
    http::{HeaderMap, StatusCode},
    response::Response,
    routing::put,
    Router,
};

use crate::utils::{get_extension, random_string};

async fn upload_screenshot(headers: HeaderMap, mut multipart: Multipart) -> Response {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if headers.get("Authorization").unwrap() != env::var("IMG_ACCESS_KEY").unwrap().as_str() {
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from(""))
                .unwrap();
        }

        if data.len() > env::var("IMG_MAX_SIZE").unwrap().parse::<usize>().unwrap() {
            return Response::builder()
                .status(StatusCode::PAYLOAD_TOO_LARGE)
                .body(Body::from(""))
                .unwrap();
        }

        let sanitized_name = format!("{}.{}", random_string(6), get_extension(file_name.as_str()));

        std::fs::write(format!("data/{}", &sanitized_name), data).unwrap();

        return Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(format!(
                "https://{}/{}",
                env::var("IMG_DOMAIN").unwrap(),
                &sanitized_name
            )))
            .unwrap();
    }

    return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from(""))
        .unwrap();
}

pub fn serve() -> Router {
    Router::new().route("/api/sxfu", put(upload_screenshot))
}
