use crate::client::HttpClient;
use crate::scanners::VulnerabilityScanner;
use shadowprobe_core::{
    HttpMethod, Result, Severity, Vulnerability, VulnerabilityType,
};
use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use tracing::debug;
use uuid::Uuid;

pub struct CorsScanner;

impl CorsScanner {
    pub fn new() -> Self {
        Self
    }

    async fn test_cors_misconfiguration(
        &self,
        client: &HttpClient,
        url: &str,
    ) -> Result<Vec<Vulnerability>> {
        let mut vulnerabilities = Vec::new();

        let test_origins = vec![
            "https://evil.com",
            "http://attacker.com",
            "null",
            "http://localhost",
        ];

        for origin in test_origins {
            let mut headers = HashMap::new();
            headers.insert("Origin".to_string(), origin.to_string());

            debug!("Testing CORS with origin: {} on {}", origin, url);

            match client.request(&HttpMethod::GET, url, Some(&headers), None).await {
                Ok(response) => {
                    if let Some(acao) = response.headers.get("access-control-allow-origin") {
                        // Check for dangerous CORS configurations
                        if acao == "*"
                            || acao == origin
                            || acao == "null"
                            || self.is_reflected_origin(&response.headers, origin) {

                            let severity = if acao == "*" || acao == "null" {
                                Severity::High
                            } else {
                                Severity::Medium
                            };

                            let has_credentials = response.headers
                                .get("access-control-allow-credentials")
                                .map(|v| v == "true")
                                .unwrap_or(false);

                            let description = if has_credentials && (acao == "*" || acao == origin) {
                                format!(
                                    "Critical CORS misconfiguration: Access-Control-Allow-Origin is '{}' with credentials enabled. This allows any origin to access sensitive data.",
                                    acao
                                )
                            } else if acao == "*" {
                                "CORS misconfiguration: Access-Control-Allow-Origin is set to wildcard (*), allowing any origin to access resources.".to_string()
                            } else {
                                format!(
                                    "CORS misconfiguration: Origin '{}' is reflected in Access-Control-Allow-Origin header.",
                                    origin
                                )
                            };

                            vulnerabilities.push(Vulnerability {
                                id: Uuid::new_v4().to_string(),
                                vuln_type: VulnerabilityType::CORS,
                                severity: if has_credentials { Severity::Critical } else { severity },
                                url: url.to_string(),
                                method: HttpMethod::GET,
                                parameter: None,
                                payload: format!("Origin: {}", origin),
                                evidence: format!(
                                    "Access-Control-Allow-Origin: {}\nAccess-Control-Allow-Credentials: {}",
                                    acao,
                                    if has_credentials { "true" } else { "false" }
                                ),
                                description,
                                remediation: "Use a whitelist of allowed origins. Never use wildcard (*) with credentials. Properly validate the Origin header.".to_string(),
                                poc: Some(format!(
                                    "curl -H 'Origin: {}' -v '{}'",
                                    origin, url
                                )),
                                ai_confidence: None,
                                ai_analysis: None,
                                timestamp: Utc::now(),
                            });
                        }
                    }
                }
                Err(e) => {
                    debug!("Error testing CORS on {}: {}", url, e);
                }
            }
        }

        Ok(vulnerabilities)
    }

    fn is_reflected_origin(&self, headers: &HashMap<String, String>, origin: &str) -> bool {
        headers
            .get("access-control-allow-origin")
            .map(|v| v.contains(origin))
            .unwrap_or(false)
    }
}

#[async_trait]
impl VulnerabilityScanner for CorsScanner {
    fn name(&self) -> &str {
        "CORS Misconfiguration Scanner"
    }

    fn vuln_type(&self) -> VulnerabilityType {
        VulnerabilityType::CORS
    }

    async fn scan(&self, client: &HttpClient, url: &str) -> Result<Vec<Vulnerability>> {
        self.test_cors_misconfiguration(client, url).await
    }
}

impl Default for CorsScanner {
    fn default() -> Self {
        Self::new()
    }
}
