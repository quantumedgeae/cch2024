use axum::{body::Bytes, http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    #[serde(flatten)]
    extra: Value,
}

// Define the private key
const PRIVATE_KEY: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQClNGmBEc3LOJgb
dDCb7g7UTDtqTNJ6UWA1JZOzUdcdaa/naThU2GhNDa7zAr5CpVwmcprhBj9boQRm
CNFJhA8M2nIHxL2YA8GHkjLYpFDkZc0DMjb16ARGksUoI1S3nN42aRlV9PGW/YdK
H7laZsSJaK7URAcylNi2OiuzCEI3O/AQYcdxbsZx8N/awBSewKoUSg5ivTQciOjx
t39nov2XPJHky2xz2+ePhfTENFz5ha5iWu08DuDLu+maw2s05r5rKB5RBdmKYAaW
67HUZd22rBl6TpdVQBqAPZQMcnlSoLjCqS5SwLaSK66JPuUNv3WYPAVO8fuCRJu3
/iyp75zlAgMBAAECggEAQjamjqevHuN4+4/vJ6Aba3V+XtWZipLxB3wfHcEQgeZp
LUvb2w29iByS4S1ZSmbybyqB7uzNGhGILMkd8OQwEAvBpmFxR+tBdu/e1zvMfC5d
JAnNQ3QRuqN6x+KZQtppqc8n6264IbdNsSTT6mrEHPrLD6KQa/KChU79Us1dlLsZ
WyuDwT5fs0TI2+84pRtEse9B/W5tyb7P+dMPpen9gY5g6bNT807rvjEt85mkL0Tc
1WTm2RAeVKjkDDu18i1HLlHkcf/hU63Wq1G76DvxHArFOfZoWp0dMEGAsMAi2Qfg
E5JcJ/ik9Vcw3MfSLHso5noZrvttTDbG0Ntdy8RgJQKBgQDO3oM4NPxrV9JOYRva
mYdqTtySv5aATRRb73VIvj8VqgaeSmfyn3VYcaxgS49c71HwIKW8F+wObC/kkSns
qovVxA4fNtbT7iF9pUqv0u86ytiqA4Kru8L1yhsNc66ga2NHRv3ZRLAjZ/SYxWcc
UYaTfKbBauoQ7GiDLWetUmO8pwKBgQDMcLuXYt1MztsEWtVZ1o9vfufTe04fgzI4
xu8YLRyl+rhM+Uig/jFpyb36SmTGkXKnL2q/xnUGqb3aq41oTQ9VlCeLure/EBWe
F18cyckdMg6o3vnSOveG103Chdl/SVs8aVArkAzNxyUZ/r3SL5zG1388zfb5lo0O
EWJNBY+PkwKBgDoKzJ50n24u0d8T+BC5ivkU72ZtQvj0p4W0BiBG4MRfVEQNdGTq
wK30Fv1dtffJXiAVyOqhe6YATN/AqxawM1PjEmNHq7cC1Xq/adkzQdQpHel12q/8
mpMEBdscXh68grp4/Lh9QT5t6H8ExA16AlxXWSJjsRDWg+xqTPCgcUN5AoGAV40H
qSM7poMHpGvJ5nlOX+83FIg/wpsKzOnUxsO6+xHOD5jKE7gILUeqRh/J3Ie5VAOo
cYxp3XqfxZ+mJT6rDHkwHlTOD9Kxj0xu/QMlT416XbNvxSpfiq11rZJovGwL2Gk5
Xg6IjS91ohZH8FVXjIgJA18pH2Xr7lRfhqCVBGUCgYAo6nmzbLG4hYq90/VKdHBq
EfZIE4uD3n2J6jzjit9/08j+q2s+tiBtmvEmSGJH0a/m6K+7I4Bh5BFEnAu5IyZH
mOtvKBxBuZCP3aQFX9bFgMJP6cJbYgWdPsbagBnbgPawguEg36OvyYpdvVg3yO6Z
yUQklDDO0v28INL5vn37pA==
-----END PRIVATE KEY-----
"#;

pub async fn wrap(cookies: Cookies, Json(payload): Json<Value>) -> impl IntoResponse {
    let encoded_value = jsonwebtoken::encode(
        &Header::new(Algorithm::RS256),
        &Claims { extra: payload },
        &EncodingKey::from_rsa_pem(PRIVATE_KEY.as_bytes()).unwrap(),
    )
    .unwrap();
    cookies.add(Cookie::new("gift", format!("({})", encoded_value)));
    StatusCode::OK
}

// Define the public key
const PUBLIC_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEApTRpgRHNyziYG3Qwm+4O
1Ew7akzSelFgNSWTs1HXHWmv52k4VNhoTQ2u8wK+QqVcJnKa4QY/W6EEZgjRSYQP
DNpyB8S9mAPBh5Iy2KRQ5GXNAzI29egERpLFKCNUt5zeNmkZVfTxlv2HSh+5WmbE
iWiu1EQHMpTYtjorswhCNzvwEGHHcW7GcfDf2sAUnsCqFEoOYr00HIjo8bd/Z6L9
lzyR5Mtsc9vnj4X0xDRc+YWuYlrtPA7gy7vpmsNrNOa+aygeUQXZimAGluux1GXd
tqwZek6XVUAagD2UDHJ5UqC4wqkuUsC2kiuuiT7lDb91mDwFTvH7gkSbt/4sqe+c
5QIDAQAB
-----END PUBLIC KEY-----
"#;

pub async fn unwrap(cookies: Cookies) -> impl IntoResponse {
    let gift_cookie_option = cookies.get("gift");
    match gift_cookie_option {
        Some(gift_cookie) => {
            let val = gift_cookie.value();
            let encoded_val = &val[1..val.len() - 1];
            // println!("{}", encoded_val);
            decode_val(encoded_val, PUBLIC_KEY.as_bytes(), Algorithm::RS256)
        }
        None => (StatusCode::BAD_REQUEST, "".to_owned()),
    }
}

fn decode_val(data_str: &str, public_key: &[u8], algo: Algorithm) -> (StatusCode, String) {
    // Create decoding key from PEM
    let decoding_key = DecodingKey::from_rsa_pem(public_key).unwrap();
    // Set up validation
    let mut validation = Validation::new(algo);
    // skip exp validation, which is on by default
    validation.required_spec_claims = HashSet::new();
    let token_data = jsonwebtoken::decode::<Claims>(data_str, &decoding_key, &validation);
    match token_data {
        Ok(token_data) => (StatusCode::OK, Json(token_data.claims.extra).to_string()),
        Err(error) if *error.kind() == ErrorKind::InvalidSignature => {
            (StatusCode::UNAUTHORIZED, "".to_owned())
        }
        Err(_) => (StatusCode::BAD_REQUEST, "".to_owned()),
    }
}

// Define the special decoding public key
const SPECIAL_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAs5BlLjDtKuEY2NV3+xhH
WWlKrZDWkIOV+HoLURIBEpAHa11xU+wL9sySR17j4bL9MJawlCJAGArW8vnDiAv8
366PfOhCqZsD9N2iG28y7vf5q1PhoXl/Vfuelykw0k+r4054h0uCg9Olal0Nm/V8
vsdPEC3wjNLBi86oYESkW43/7lbBWPBti1POCVJDuBEASZFhIR2+mfz6AFWQwmqO
zzhP1Yli/7EtNMELWezQJXnVLQ3JvjT2btWWwKYT468YX/NtQgMC7SLvIRBuWb/Z
ayfoi/9rGndqW0YPE1xwJEQA415w5HbfTneyAIxDy7TC8/+dFaKRcoPiEQA1T5bk
OQIDAQAB
-----END PUBLIC KEY-----
"#;

pub async fn decode(data: Bytes) -> impl IntoResponse {
    // Decode and verify the token
    let data_str: &str = unsafe { std::str::from_utf8_unchecked(data.as_ref()) };
    // println!("{}", data_str);
    decode_val(data_str, SPECIAL_KEY.as_bytes(), Algorithm::RS256)
}
