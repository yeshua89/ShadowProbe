pub mod sqli;
pub mod xss;
pub mod ssrf;
pub mod lfi;
pub mod ssti;
pub mod command_injection;
pub mod xxe;
pub mod cors;
pub mod open_redirect;

use crate::client::HttpClient;
use shadowprobe_core::{Vulnerability, VulnerabilityType, Result};
use async_trait::async_trait;

#[async_trait]
pub trait VulnerabilityScanner: Send + Sync {
    fn name(&self) -> &str;
    fn vuln_type(&self) -> VulnerabilityType;
    async fn scan(&self, client: &HttpClient, url: &str) -> Result<Vec<Vulnerability>>;
}

pub struct ScannerEngine {
    scanners: Vec<Box<dyn VulnerabilityScanner>>,
}

impl ScannerEngine {
    pub fn new() -> Self {
        let scanners: Vec<Box<dyn VulnerabilityScanner>> = vec![
            Box::new(sqli::SqlInjectionScanner::new()),
            Box::new(xss::XssScanner::new()),
            Box::new(ssrf::SsrfScanner::new()),
            Box::new(lfi::LfiScanner::new()),
            Box::new(ssti::SstiScanner::new()),
            Box::new(command_injection::CommandInjectionScanner::new()),
            Box::new(xxe::XxeScanner::new()),
            Box::new(cors::CorsScanner::new()),
            Box::new(open_redirect::OpenRedirectScanner::new()),
        ];

        Self { scanners }
    }

    pub async fn scan_url(
        &self,
        client: &HttpClient,
        url: &str,
    ) -> Result<Vec<Vulnerability>> {
        let mut all_vulns = Vec::new();

        for scanner in &self.scanners {
            let vulns = scanner.scan(client, url).await?;
            all_vulns.extend(vulns);
        }

        Ok(all_vulns)
    }

    pub fn scanner_count(&self) -> usize {
        self.scanners.len()
    }
}

impl Default for ScannerEngine {
    fn default() -> Self {
        Self::new()
    }
}
