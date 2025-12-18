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

pub struct OpenRedirectScanner {
    payloads: PayloadSet,
}

impl OpenRedirectScanner {
    pub fn new() -> Self {
        Self {
            payloads: PayloadSet::for_open_redirect(),
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

            debug!("Testing Open Redirect with payload: {} on {}", payload.description, test_url);

            match client.get(&test_url).await {
                Ok(response) => {
                    // Check if redirect to external domain
                    if self.is_open_redirect(&response, &payload.value) {
                        vulnerabilities.push(Vulnerability {
                            id: Uuid::new_v4().to_string(),
                            vuln_type: VulnerabilityType::OpenRedirect,
                            severity: Severity::Medium,
                            url: test_url.clone(),
                            method: HttpMethod::GET,
                            parameter: Some(param.to_string()),
                            payload: payload.value.clone(),
                            evidence: format!(
                                "Status: {}, Location: {}",
                                response.status,
                                response.headers.get("location").unwrap_or(&"N/A".to_string())
                            ),
                            description: format!(
                                "Open Redirect vulnerability detected using {}. The application redirects to user-controlled URLs.",
                                payload.description
                            ),
                            remediation: "Use a whitelist of allowed redirect destinations. Validate all redirect URLs. Avoid using user input directly in redirect targets.".to_string(),
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

    fn is_open_redirect(&self, response: &crate::client::HttpResponse, payload: &str) -> bool {
        // Check for 3xx redirect status
        if !response.is_redirect() {
            return false;
        }

        // Check Location header
        if let Some(location) = response.headers.get("location") {
            // Check if location contains our payload or points to external domain
            if location.contains("evil.com")
                || location.contains(payload)
                || location.starts_with("//")
                || location.starts_with("javascript:") {
                return true;
            }

            // Parse and check if it's external
            if let Ok(loc_url) = Url::parse(location) {
                if let Some(host) = loc_url.host_str() {
                    return host.contains("evil.com") || host != "target.com";
                }
            }
        }

        false
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
impl VulnerabilityScanner for OpenRedirectScanner {
    fn name(&self) -> &str {
        "Open Redirect Scanner"
    }

    fn vuln_type(&self) -> VulnerabilityType {
        VulnerabilityType::OpenRedirect
    }

    async fn scan(&self, client: &HttpClient, url: &str) -> Result<Vec<Vulnerability>> {
        let mut all_vulns = Vec::new();

        let params = Self::extract_parameters(url);
        if params.is_empty() {
            let test_params = vec!["redirect", "url", "next", "return", "dest", "destination", "redir", "redirect_uri"];
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

impl Default for OpenRedirectScanner {
    fn default() -> Self {
        Self::new()
    }
}
