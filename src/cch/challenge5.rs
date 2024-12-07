use axum::{
    body::Bytes,
    http::{header::CONTENT_TYPE, HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde_json::{self, Error, Value};
use std::{borrow::Cow, fmt::Display};
use toml::Table;

const TOML_MIME_TYPE: &str = "application/toml";
const JSON_MIME_TYPE: &str = "application/json";
const YAML_MIME_TYPE: &str = "application/yaml";

#[derive(Debug, Default, PartialEq)]
struct Validation {
    status_code: StatusCode,
    header: HeaderMap,
    body: String,
}

struct Order {
    item: String,
    quantity: u32,
}
impl TryFrom<&toml::Value> for Order {
    type Error = ();

    fn try_from(value: &toml::Value) -> Result<Self, Self::Error> {
        let value = value.as_table().unwrap();
        if value.contains_key("item") && value.contains_key("quantity") {
            let x = value["item"].as_str().ok_or(())?;
            let item = x.to_owned();
            let quantity = value["quantity"].as_integer().ok_or(())? as u32;
            return Ok(Self { item, quantity });
        }
        Err(())
    }
}
impl TryFrom<&serde_json::Value> for Order {
    type Error = ();

    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let value = value.as_object().unwrap();
        if value.contains_key("item") && value.contains_key("quantity") {
            let x = value["item"].as_str().ok_or(())?;
            let item = x.to_owned();
            let quantity = value["quantity"].as_u64().ok_or(())? as u32;
            return Ok(Self { item, quantity });
        }
        Err(())
    }
}
impl TryFrom<&serde_yaml::Value> for Order {
    type Error = ();

    fn try_from(value: &serde_yaml::Value) -> Result<Self, Self::Error> {
        let value = value.as_mapping().unwrap();
        if value.contains_key("item") && value.contains_key("quantity") {
            let x = value["item"].as_str().ok_or(())?;
            let item = x.to_owned();
            let quantity = value["quantity"].as_u64().ok_or(())? as u32;
            return Ok(Self { item, quantity });
        }
        Err(())
    }
}
impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.item, self.quantity)
    }
}
struct Orders(Vec<Order>);
impl Display for Orders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut comma_separated = String::new();

        for num in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str("\n");
        }

        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "{}", comma_separated)
    }
}

fn validate(headers: Option<HeaderMap>, data: Bytes) -> Validation {
    let mut parsed_orders = Orders(vec![]);
    let headers = headers.unwrap();
    if let Some(header_type) = headers.get(CONTENT_TYPE) {
        let utf8_str = std::str::from_utf8(data.as_ref()).unwrap();
        // println!("{}", utf8_str);
        if [TOML_MIME_TYPE, JSON_MIME_TYPE, YAML_MIME_TYPE].contains(&header_type.to_str().unwrap())
        {
            let value = String::from(Cow::from(utf8_str)).parse::<Table>();
            if let Ok(toml_table) = value {
                if toml_table.contains_key("package") {
                    // println!("got package");
                    let package_table = toml_table["package"].as_table().unwrap();
                    if package_table.contains_key("metadata") {
                        // println!("got metadata");
                        let metadata_table = package_table["metadata"].as_table().unwrap();
                        if metadata_table.contains_key("orders") {
                            // println!("got orders");
                            let orders_array = metadata_table["orders"].as_array().unwrap();
                            // println!("{:?}", orders_array);
                            for order_value in orders_array {
                                // let order_value: &toml::Value = order_value;
                                if let Ok(order) = TryInto::<Order>::try_into(order_value) {
                                    // println!("{:?}", order);
                                    parsed_orders.0.push(order);
                                }
                            }
                        } else {
                            return Validation {
                                status_code: StatusCode::BAD_REQUEST,
                                header: HeaderMap::new(),
                                body: String::from(""),
                            };
                        }
                    } else {
                        return Validation {
                            status_code: StatusCode::BAD_REQUEST,
                            header: HeaderMap::new(),
                            body: String::from(""),
                        };
                    }
                } else {
                    return Validation {
                        status_code: StatusCode::BAD_REQUEST,
                        header: HeaderMap::new(),
                        body: String::from(""),
                    };
                }
                // println!("{}", toml_table);
                // let res = panic::catch_unwind(|| {
                //     println!("{}", toml_table["package"]["metadata"]["orders"])
                // });
            } else {
                // println!("NO json ORDERS");
                let json_table: Result<Value, Error> = serde_json::from_str(utf8_str);
                if let Ok(json_value) = json_table {
                    if let Some(json_table) = json_value.as_object() {
                        if json_table.contains_key("package") {
                            // println!("got package");
                            let package_table = json_table["package"].as_object().unwrap();
                            if package_table.contains_key("metadata") {
                                // println!("got metadata");
                                let metadata_table = package_table["metadata"].as_object().unwrap();
                                if metadata_table.contains_key("orders") {
                                    // println!("got orders");
                                    let orders_array = metadata_table["orders"].as_array().unwrap();
                                    // println!("{:?}", orders_array);
                                    for order_value in orders_array {
                                        // let order_value: &toml::Value = order_value;
                                        if let Ok(order) = TryInto::<Order>::try_into(order_value) {
                                            // println!("{:?}", order);
                                            parsed_orders.0.push(order);
                                        }
                                    }
                                } else {
                                    return Validation {
                                        status_code: StatusCode::BAD_REQUEST,
                                        header: HeaderMap::new(),
                                        body: String::from(""),
                                    };
                                }
                            } else {
                                return Validation {
                                    status_code: StatusCode::BAD_REQUEST,
                                    header: HeaderMap::new(),
                                    body: String::from(""),
                                };
                            }
                        } else {
                            return Validation {
                                status_code: StatusCode::BAD_REQUEST,
                                header: HeaderMap::new(),
                                body: String::from(""),
                            };
                        }
                    }
                    // println!("{}", json_value["package"]["metadata"]["orders"]);
                } else {
                    // println!("NO yaml ORDERS");
                    let yaml_table: Result<serde_yaml::Value, serde_yaml::Error> =
                        serde_yaml::from_str(utf8_str);
                    if let Ok(yaml_value) = yaml_table {
                        if let Some(yaml_table) = yaml_value.as_mapping() {
                            if yaml_table.contains_key("package") {
                                // println!("got package");
                                let package_table = yaml_table["package"].as_mapping().unwrap();
                                if package_table.contains_key("metadata") {
                                    // println!("got metadata");
                                    let metadata_table =
                                        package_table["metadata"].as_mapping().unwrap();
                                    if metadata_table.contains_key("orders") {
                                        // println!("got orders");
                                        let orders_array =
                                            metadata_table["orders"].as_sequence().unwrap();
                                        // println!("{:?}", orders_array);
                                        for order_value in orders_array {
                                            // let order_value: &toml::Value = order_value;
                                            if let Ok(order) =
                                                TryInto::<Order>::try_into(order_value)
                                            {
                                                // println!("{:?}", order);
                                                parsed_orders.0.push(order);
                                            }
                                        }
                                    } else {
                                        return Validation {
                                            status_code: StatusCode::BAD_REQUEST,
                                            header: HeaderMap::new(),
                                            body: String::from(""),
                                        };
                                    }
                                } else {
                                    return Validation {
                                        status_code: StatusCode::BAD_REQUEST,
                                        header: HeaderMap::new(),
                                        body: String::from(""),
                                    };
                                }
                            } else {
                                return Validation {
                                    status_code: StatusCode::BAD_REQUEST,
                                    header: HeaderMap::new(),
                                    body: String::from(""),
                                };
                            }
                        }
                        // println!("{:?}", yaml_value["package"]["metadata"]["orders"]);
                    }
                }
                // println!("{}", json_table);
            }
            if parsed_orders.0.is_empty() {
                return Validation {
                    status_code: StatusCode::NO_CONTENT,
                    header: HeaderMap::new(),
                    body: String::from(""),
                };
            }
            return Validation {
                status_code: StatusCode::OK,
                header: HeaderMap::new(),
                body: parsed_orders.to_string(),
            };
        } else {
            Validation {
                status_code: StatusCode::UNSUPPORTED_MEDIA_TYPE,
                header: HeaderMap::new(),
                body: String::from(""),
            }
        }
    } else {
        Validation {
            status_code: StatusCode::NO_CONTENT,
            header: HeaderMap::new(),
            body: String::from(""),
        }
    }
}

pub async fn manifest_messaging(headers: HeaderMap, data: Bytes) -> impl IntoResponse {
    // println!("Body: {:?}", data);
    let validation = validate(Some(headers), data);
    (validation.status_code, validation.header, validation.body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    const HTML_MIME_TYPE: &str = "text/html";

    fn header_content_type(content_type: &str) -> Option<HeaderMap> {
        let mut hm = HeaderMap::new();
        hm.insert(CONTENT_TYPE, HeaderValue::from_str(content_type).unwrap());
        Some(hm)
    }

    #[test]
    fn test_task1_good() {
        let data = b"
[package]
name = \"not-a-gift-order\"
authors = [\"Not Santa\"]
keywords = [\"Christmas 2024\"]

[[package.metadata.orders]]
item = \"Toy car\"
quantity = 2

[[package.metadata.orders]]
item = \"Lego brick\"
quantity = 23
";
        let validated = validate(
            header_content_type(TOML_MIME_TYPE),
            Bytes::from_static(data),
        );
        assert_eq!(StatusCode::OK, validated.status_code);
        assert_eq!("Toy car: 2\nLego brick: 23", validated.body);
    }

    #[test]
    fn test_task1_bad() {
        let data = b"
[package]
name = \"coal-in-a-bowl\"
authors = [\"H4CK3R_13E7\"]
keywords = [\"Christmas 2024\"]

[[package.metadata.orders]]
item = \"Coal\"
quantity = \"Hahaha get rekt\"
";
        assert_eq!(
            StatusCode::NO_CONTENT,
            validate(
                header_content_type(TOML_MIME_TYPE),
                Bytes::from_static(data)
            )
            .status_code
        );
    }

    #[test]
    fn test_task2_fail1() {
        let data = b"
[package]
name = false
authors = [\"Not Santa\"]
keywords = [\"Christmas 2024\"]
";
        assert_eq!(
            StatusCode::BAD_REQUEST,
            validate(
                header_content_type(TOML_MIME_TYPE),
                Bytes::from_static(data)
            )
            .status_code
        );
    }

    #[test]
    fn test_task2_fail2() {
        let data = b"
[package]
name = \"not-a-gift-order\"
authors = [\"Not Santa\"]
keywords = [\"Christmas 2024\"]

[profile.release]
incremental = \"stonks\"
";
        assert_eq!(
            StatusCode::BAD_REQUEST,
            validate(
                header_content_type(TOML_MIME_TYPE),
                Bytes::from_static(data)
            )
            .status_code
        );
    }

    #[test]
    fn test_task3_fail1() {
        let data = b"
[package]
name = \"grass\"
authors = [\"A vegan cow\"]
keywords = [\"Moooooo\"]
";
        assert_eq!(
            StatusCode::BAD_REQUEST,
            validate(
                header_content_type(TOML_MIME_TYPE),
                Bytes::from_static(data)
            )
            .status_code
        );
    }

    #[test]
    fn test_task4_html_bad() {
        let data = b"<h1>Hello, bird!</h1>";
        assert_eq!(
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            validate(
                header_content_type(HTML_MIME_TYPE),
                Bytes::from_static(data)
            )
            .status_code
        );
    }

    #[test]
    fn test_task4_yaml_good() {
        let data = b"
package:
  name: big-chungus-sleigh
  version: \"2.0.24\"
  metadata:
    orders:
      - item: \"Toy train\"
        quantity: 5
  rust-version: \"1.69\"
  keywords:
    - \"Christmas 2024\"
";
        assert_eq!(
            StatusCode::OK,
            validate(
                header_content_type(TOML_MIME_TYPE),
                Bytes::from_static(data)
            )
            .status_code
        );
    }

    #[test]
    fn test_task4_json_good() {
        let data = b"
{
  \"package\": {
    \"name\": \"big-chungus-sleigh\",
    \"version\": \"2.0.24\",
    \"metadata\": {
      \"orders\": [
        {
          \"item\": \"Toy train\",
          \"quantity\": 5
        }
      ]
    },
    \"rust-version\": \"1.69\",
    \"keywords\": [
      \"Christmas 2024\"
    ]
  }
}
";
        assert_eq!(
            StatusCode::OK,
            validate(
                header_content_type(TOML_MIME_TYPE),
                Bytes::from_static(data)
            )
            .status_code
        );
    }
}
