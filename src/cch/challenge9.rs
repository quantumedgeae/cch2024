// use crate::AppState;
use axum::{
    body::Bytes,
    http::{header::CONTENT_TYPE, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    // Extension,
};
use serde_json::{Error, Map, Value};
use std::cmp::Ordering;

const JSON_MIME_TYPE: &str = "application/json";
const JSON_TYPE_LITERS: &str = "liters";
const JSON_TYPE_GALLONS: &str = "gallons";
const JSON_TYPE_LITRES: &str = "litres";
const JSON_TYPE_PINTS: &str = "pints";

pub async fn milk(
    // Extension(_): Extension<Arc<AppState>>,
    headers: HeaderMap,
    data: Bytes,
) -> impl IntoResponse {
    if let Some(header_type) = headers.get(CONTENT_TYPE) {
        if header_type.to_str().unwrap().cmp(JSON_MIME_TYPE) == Ordering::Equal {
            let utf8_str = std::str::from_utf8(data.as_ref()).unwrap();
            let json_result: Result<Map<String, Value>, Error> = serde_json::from_str(utf8_str);
            if let Ok(json_map) = json_result {
                // println!("{:?}", json_map);
                if json_map.len() > 1 {
                    return (StatusCode::BAD_REQUEST, HeaderMap::new(), "< ".to_owned());
                }
                let key = json_map.keys().next().unwrap().clone();
                if [
                    JSON_TYPE_LITERS,
                    JSON_TYPE_GALLONS,
                    JSON_TYPE_LITRES,
                    JSON_TYPE_PINTS,
                ]
                .contains(&key.as_ref())
                {
                    let val = json_map.get(&key).unwrap().as_f64().unwrap() as f32;

                    let mut converted_val = val;
                    let mut type_shown = "";
                    let mut width = 7;
                    if key == JSON_TYPE_LITERS {
                        converted_val = val * 0.26417206;
                        type_shown = JSON_TYPE_GALLONS;
                        width = 7;
                    }
                    if key == JSON_TYPE_GALLONS {
                        converted_val = val * 3.785412;
                        type_shown = JSON_TYPE_LITERS;
                        width = 5;
                    }
                    if key == JSON_TYPE_LITRES {
                        converted_val = val * 1.759754;
                        type_shown = JSON_TYPE_PINTS;
                        width = 6;
                    }
                    if key == JSON_TYPE_PINTS {
                        converted_val = val * 0.56826125;
                        type_shown = JSON_TYPE_LITRES;
                        width = 7;
                    }

                    // println!("{}:{}", &key, converted_val);
                    let converted_info = format!(
                        "{{\"{}\":{:.width$}}}\n",
                        type_shown,
                        converted_val,
                        width = width,
                    );
                    let mut json_header_map = HeaderMap::new();
                    json_header_map
                        .insert(CONTENT_TYPE, HeaderValue::from_str(JSON_MIME_TYPE).unwrap());
                    return (StatusCode::OK, json_header_map, converted_info);
                } else {
                    return (StatusCode::BAD_REQUEST, HeaderMap::new(), "< ".to_owned());
                }
            }
        }
    }
    (
        StatusCode::OK,
        HeaderMap::new(),
        "Milk withdrawn\n".to_owned(),
    )
}
