use core::str;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use axum::{extract::Query, http::header, response::IntoResponse};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IPParams {
    from: String,
    key: Option<String>,
    to: Option<String>,
}

pub async fn calc_ip_ops(ip_params: Query<IPParams>) -> impl IntoResponse {
    let ip_params: IPParams = ip_params.0;
    let from_addr = ip_params.from.parse::<IpAddr>().unwrap();
    // println!("{from_addr:?}");
    let mut res: Option<String> = None;

    if from_addr.is_ipv6() {
        let mut other_addr = None;
        if let Some(ref key) = ip_params.key {
            other_addr = Some(key.parse::<Ipv6Addr>().unwrap());
        }
        if let Some(ref to) = ip_params.to {
            other_addr = Some(to.parse::<Ipv6Addr>().unwrap());
        }
        let from_addr: Ipv6Addr = from_addr.to_string().parse().unwrap();
        let from_octets = from_addr.octets();
        let key_octets = other_addr.unwrap().octets();
        let res_ipv6_addr = Ipv6Addr::new(
            (from_octets[0] as u16 * 256 + from_octets[1] as u16)
                ^ (key_octets[0] as u16 * 256 + key_octets[1] as u16),
            (from_octets[2] as u16 * 256 + from_octets[3] as u16)
                ^ (key_octets[2] as u16 * 256 + key_octets[3] as u16),
            (from_octets[4] as u16 * 256 + from_octets[5] as u16)
                ^ (key_octets[4] as u16 * 256 + key_octets[5] as u16),
            (from_octets[6] as u16 * 256 + from_octets[7] as u16)
                ^ (key_octets[6] as u16 * 256 + key_octets[7] as u16),
            (from_octets[8] as u16 * 256 + from_octets[9] as u16)
                ^ (key_octets[8] as u16 * 256 + key_octets[9] as u16),
            (from_octets[10] as u16 * 256 + from_octets[11] as u16)
                ^ (key_octets[10] as u16 * 256 + key_octets[11] as u16),
            (from_octets[12] as u16 * 256 + from_octets[13] as u16)
                ^ (key_octets[12] as u16 * 256 + key_octets[13] as u16),
            (from_octets[14] as u16 * 256 + from_octets[15] as u16)
                ^ (key_octets[14] as u16 * 256 + key_octets[15] as u16),
        );
        res = Some(res_ipv6_addr.to_string());
    } else {
        if let Some(ref key) = ip_params.key {
            let key_addr = key.parse::<IpAddr>().unwrap();
            let from_addr: Ipv4Addr = from_addr.to_string().parse().unwrap();
            let from_octets = from_addr.octets();
            let key_addr: Ipv4Addr = key_addr.to_string().parse().unwrap();
            let key_octets = key_addr.octets();
            let mut str_res = String::new();
            for i in 0..from_octets.len() {
                str_res += from_octets[i]
                    .wrapping_add(key_octets[i])
                    .to_string()
                    .as_str();
                if i != from_octets.len() - 1 {
                    str_res += ".";
                }
            }
            res = Some(str_res.to_owned());
        }
        if let Some(ref to) = ip_params.to {
            let to_addr = to.parse::<IpAddr>().unwrap();
            let from_addr: Ipv4Addr = from_addr.to_string().parse().unwrap();
            let from_octets = from_addr.octets();
            let to_addr: Ipv4Addr = to_addr.to_string().parse().unwrap();
            let to_octets = to_addr.octets();
            let mut str_res = String::new();
            for i in 0..from_octets.len() {
                str_res += to_octets[i]
                    .wrapping_sub(from_octets[i])
                    .to_string()
                    .as_str();
                if i != from_octets.len() - 1 {
                    str_res += ".";
                }
            }
            res = Some(str_res.to_owned());
        }
    }

    // println!("{res:?}");
    ([(header::CONTENT_TYPE, "text/plain")], res.unwrap())
}
