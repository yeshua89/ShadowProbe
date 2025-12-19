use crate::client::HttpClient;
use crate::payloads::PayloadSet;
use crate::scanners::VulnerabilityScanner;
use shadowprobe_core::{
    HttpMethod, Result, Severity, Vulnerability, VulnerabilityType,
};
use async_trait::async_trait;
use chrono::Utc;
use tracing::debug;
use url::Url;
use uuid::Uuid;

pub struct XssScanner {
    payloads: PayloadSet,
}

impl XssScanner {
    pub fn new() -> Self {
        Self {
            payloads: PayloadSet::for_xss(),
        }
    }

    async fn test_parameter(
        &self,
        client: &HttpClient,
        base_url: &str,
        param: &str,
    ) -> Result<Vec<Vulnerability>> {
        let mut vulnerabilities = Vec::new();

        for payload in &self.payloads.payloads {
            let test_url = if base_url.contains('?') {
                format!("{}&{}={}", base_url, param, urlencoding::encode(&payload.value))
            } else {
                format!("{}?{}={}", base_url, param, urlencoding::encode(&payload.value))
            };

            debug!("Testing XSS with payload: {} on {}", payload.description, test_url);

            match client.get(&test_url).await {
                Ok(response) => {
                    if self.is_vulnerable(&response.body, &payload.detection_patterns) {
                        vulnerabilities.push(Vulnerability {
                            id: Uuid::new_v4().to_string(),
                            vuln_type: VulnerabilityType::XSS,
                            severity: Severity::High,
                            url: test_url.clone(),
                            method: HttpMethod::GET,
                            parameter: Some(param.to_string()),
                            payload: payload.value.clone(),
                            evidence: self.extract_evidence(&response.body, 300),
                            description: format!(
                                "Cross-Site Scripting (XSS) detected using {}. The application reflects user input without proper sanitization.",
                                payload.description
                            ),
                            remediation: "Encode all user input before rendering in HTML context. Use Content Security Policy (CSP). Never trust user input.".to_string(),
                            poc: Some(format!("curl -v '{}'", test_url)),
                            ai_confidence: None,
                            ai_analysis: None,
                            timestamp: Utc::now(),
                        });
                    }
                }
                Err(e) => {
                    debug!("Error testing {}: {}", test_url, e);
                }
            }
        }

        Ok(vulnerabilities)
    }

    fn is_vulnerable(&self, body: &str, patterns: &[String]) -> bool {
        patterns.iter().any(|pattern| body.contains(pattern))
    }

    fn extract_evidence(&self, body: &str, max_len: usize) -> String {
        if body.len() > max_len {
            format!("{}...", &body[..max_len])
        } else {
            body.to_string()
        }
    }

    fn extract_parameters(url_str: &str) -> Vec<String> {
        if let Ok(url) = Url::parse(url_str) {
            url.query_pairs()
                .map(|(key, _)| key.to_string())
                .collect()
        } else {
            vec![]
        }
    }
}

#[async_trait]
impl VulnerabilityScanner for XssScanner {
    fn name(&self) -> &str {
        "XSS Scanner"
    }

    fn vuln_type(&self) -> VulnerabilityType {
        VulnerabilityType::XSS
    }

    async fn scan(&self, client: &HttpClient, url: &str) -> Result<Vec<Vulnerability>> {
        let mut all_vulns = Vec::new();

        let params = Self::extract_parameters(url);
        if params.is_empty() {
            let test_params = vec!["q", "search", "query", "name", "message"];
            for param in test_params {
                let vulns = self.test_parameter(client, url, param).await?;
                all_vulns.extend(vulns);
            }
        } else {
            for param in params {
                let vulns = self.test_parameter(client, url, &param).await?;
                all_vulns.extend(vulns);
            }
        }

        Ok(all_vulns)
    }
}
