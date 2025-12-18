use reqwest::{Client, Response, header};
use shadowprobe_core::{Result, ShadowProbeError, HttpMethod};
use std::time::Duration;
use std::collections::HashMap;
use tracing::{debug, warn};

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    timeout: Duration,
    max_redirects: usize,
}

impl HttpClient {
    pub fn new(timeout_secs: u64, user_agent: &str, max_redirects: usize) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(user_agent)
                .map_err(|e| ShadowProbeError::HttpError(e.to_string()))?,
        );

        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .redirect(reqwest::redirect::Policy::limited(max_redirects))
            .default_headers(headers)
            .danger_accept_invalid_certs(true) // Para testing, puede ser configurable
            .build()
            .map_err(|e| ShadowProbeError::HttpError(e.to_string()))?;

        Ok(Self {
            client,
            timeout: Duration::from_secs(timeout_secs),
            max_redirects,
        })
    }

    pub async fn request(
        &self,
        method: &HttpMethod,
        url: &str,
        headers: Option<&HashMap<String, String>>,
        body: Option<&str>,
    ) -> Result<HttpResponse> {
        debug!("Making {} request to {}", method.as_str(), url);

        let mut req = match method {
            HttpMethod::GET => self.client.get(url),
            HttpMethod::POST => self.client.post(url),
            HttpMethod::PUT => self.client.put(url),
            HttpMethod::DELETE => self.client.delete(url),
            HttpMethod::PATCH => self.client.patch(url),
            HttpMethod::HEAD => self.client.head(url),
            HttpMethod::OPTIONS => self.client.request(reqwest::Method::OPTIONS, url),
        };

        if let Some(hdrs) = headers {
            for (key, value) in hdrs {
                req = req.header(key, value);
            }
        }

        if let Some(b) = body {
            req = req.body(b.to_string());
        }

        let start = std::time::Instant::now();
        let response = req.send().await
            .map_err(|e| ShadowProbeError::HttpError(e.to_string()))?;
        let elapsed = start.elapsed();

        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let body = response.text().await
            .map_err(|e| ShadowProbeError::HttpError(e.to_string()))?;

        Ok(HttpResponse {
            status,
            headers: headers.iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect(),
            body,
            response_time: elapsed,
        })
    }

    pub async fn get(&self, url: &str) -> Result<HttpResponse> {
        self.request(&HttpMethod::GET, url, None, None).await
    }

    pub async fn post(&self, url: &str, body: &str) -> Result<HttpResponse> {
        self.request(&HttpMethod::POST, url, None, Some(body)).await
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub response_time: Duration,
}

impl HttpResponse {
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    pub fn is_redirect(&self) -> bool {
        (300..400).contains(&self.status)
    }

    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status)
    }

    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.status)
    }

    pub fn content_type(&self) -> Option<&String> {
        self.headers.get("content-type")
    }

    pub fn is_html(&self) -> bool {
        self.content_type()
            .map(|ct| ct.contains("text/html"))
            .unwrap_or(false)
    }

    pub fn is_json(&self) -> bool {
        self.content_type()
            .map(|ct| ct.contains("application/json"))
            .unwrap_or(false)
    }
}
