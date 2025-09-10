use axum::{
    extract::{ConnectInfo, FromRequestParts},
    http::{
        Extensions, StatusCode,
        header::{HeaderMap, HeaderName, USER_AGENT},
        request::Parts,
    },
};
use sqlx::types::ipnetwork::{IpNetwork, Ipv4Network, Ipv6Network};
use std::net::{IpAddr, SocketAddr};
pub const REQUEST_ID_HEADER: &str = "x-request-id";
#[derive(Debug, Clone)]
pub struct RequestMetadata {
    pub user_agent: String,
    pub ip_address: Option<IpNetwork>,
    pub request_id: String,
    pub session_id: Option<String>,
}

impl<S> FromRequestParts<S> for RequestMetadata
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract User-Agent
        let user_agent = parts
            .headers
            .get(USER_AGENT)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        let ip_string = extract_client_ip(&parts.headers, &parts.extensions)
            .unwrap_or_else(|| "unknown".to_string());

        let ip_address = if let Ok(ip) = ip_string.parse::<IpAddr>() {
            match ip {
                IpAddr::V4(ipv4) => Some(IpNetwork::V4(Ipv4Network::new(ipv4, 32).unwrap())),
                IpAddr::V6(ipv6) => Some(IpNetwork::V6(Ipv6Network::new(ipv6, 128).unwrap())),
            }
        } else {
            // Fallback to localhost if parsing fails
            let localhost = "127.0.0.1".parse::<IpAddr>().unwrap();
            match localhost {
                IpAddr::V4(ipv4) => Some(IpNetwork::V4(Ipv4Network::new(ipv4, 32).unwrap())),
                IpAddr::V6(_) => unreachable!(), // localhost is always IPv4 in this case
            }
        };
        // Generate or extract request ID
        let request_id = parts
            .headers
            .get(REQUEST_ID_HEADER)
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
            .expect("Request ID should always be set by SetRequestIdLayer");

        // Extract session ID from cookie or header
        let session_id = extract_session_id(&parts.headers);

        tracing::info!(user_agent, request_id, session_id, ip_string);
        Ok(RequestMetadata {
            user_agent,
            ip_address,
            request_id,
            session_id,
        })
    }
}

fn extract_client_ip(headers: &HeaderMap, extensions: &Extensions) -> Option<String> {
    // Priority order for IP extraction (reverse proxy aware)
    let ip_headers = [
        "x-forwarded-for",
        "x-real-ip",
        "cf-connecting-ip", // Cloudflare
        "x-client-ip",
    ];

    // Check proxy headers first
    for header_name in ip_headers {
        let header_name = HeaderName::from_static(header_name);

        if let Some(header_value) = headers.get(&header_name) {
            if let Ok(ip_str) = header_value.to_str() {
                // X-Forwarded-For can contain multiple IPs, take the first
                let ip = ip_str.split(',').next()?.trim();
                if !ip.is_empty() {
                    return Some(ip.to_string());
                }
            }
        }
    }

    // Fallback to connection info
    extensions
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(addr)| addr.ip().to_string())
}

fn extract_session_id(headers: &HeaderMap) -> Option<String> {
    // Check Authorization header first (Bearer token)
    if let Some(auth) = headers.get("authorization") {
        if let Ok(auth_str) = auth.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string());
            }
        }
    }

    // Check custom session header
    if let Some(session) = headers.get("x-session-id") {
        if let Ok(session_str) = session.to_str() {
            return Some(session_str.to_string());
        }
    }

    // Parse cookies for session
    if let Some(cookie_header) = headers.get("cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            for cookie in cookie_str.split(';') {
                let cookie = cookie.trim();
                if let Some((key, value)) = cookie.split_once('=') {
                    if key.trim() == "session_id" || key.trim() == "sessionid" {
                        return Some(value.trim().to_string());
                    }
                }
            }
        }
    }

    None
}
