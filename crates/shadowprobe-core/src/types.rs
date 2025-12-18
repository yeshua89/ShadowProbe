use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub url: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl HttpMethod {
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub fn as_str(&self) -> &str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
            Severity::Info => "INFO",
        }
    }

    pub fn color_code(&self) -> &str {
        match self {
            Severity::Critical => "\x1b[1;35m", // Magenta
            Severity::High => "\x1b[1;31m",     // Red
            Severity::Medium => "\x1b[1;33m",   // Yellow
            Severity::Low => "\x1b[1;36m",      // Cyan
            Severity::Info => "\x1b[1;37m",     // White
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VulnerabilityType {
    SQLInjection,
    XSS,
    SSRF,
    LFI,
    RFI,
    CommandInjection,
    SSTI,
    XXE,
    OpenRedirect,
    PathTraversal,
    CORS,
    CSRF,
    Deserialization,
    Authentication,
    Authorization,
    InformationDisclosure,
    Custom(String),
}

impl VulnerabilityType {
    pub fn as_str(&self) -> &str {
        match self {
            VulnerabilityType::SQLInjection => "SQL Injection",
            VulnerabilityType::XSS => "Cross-Site Scripting (XSS)",
            VulnerabilityType::SSRF => "Server-Side Request Forgery (SSRF)",
            VulnerabilityType::LFI => "Local File Inclusion (LFI)",
            VulnerabilityType::RFI => "Remote File Inclusion (RFI)",
            VulnerabilityType::CommandInjection => "Command Injection",
            VulnerabilityType::SSTI => "Server-Side Template Injection (SSTI)",
            VulnerabilityType::XXE => "XML External Entity (XXE)",
            VulnerabilityType::OpenRedirect => "Open Redirect",
            VulnerabilityType::PathTraversal => "Path Traversal",
            VulnerabilityType::CORS => "CORS Misconfiguration",
            VulnerabilityType::CSRF => "Cross-Site Request Forgery (CSRF)",
            VulnerabilityType::Deserialization => "Insecure Deserialization",
            VulnerabilityType::Authentication => "Authentication Bypass",
            VulnerabilityType::Authorization => "Authorization Bypass",
            VulnerabilityType::InformationDisclosure => "Information Disclosure",
            VulnerabilityType::Custom(name) => name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub id: String,
    pub vuln_type: VulnerabilityType,
    pub severity: Severity,
    pub url: String,
    pub method: HttpMethod,
    pub parameter: Option<String>,
    pub payload: String,
    pub evidence: String,
    pub description: String,
    pub remediation: String,
    pub poc: Option<String>,
    pub ai_confidence: Option<f32>,
    pub ai_analysis: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub target_url: String,
    pub scan_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub vulnerabilities: Vec<Vulnerability>,
    pub endpoints_discovered: Vec<String>,
    pub total_requests: u64,
    pub status: ScanStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScanStatus {
    Running,
    Completed,
    Failed,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub target: String,
    pub max_depth: usize,
    pub max_concurrent_requests: usize,
    pub timeout_seconds: u64,
    pub follow_redirects: bool,
    pub user_agent: String,
    pub custom_headers: HashMap<String, String>,
    pub scan_types: Vec<VulnerabilityType>,
    pub enable_ai_analysis: bool,
    pub aggressive_mode: bool,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            target: String::new(),
            max_depth: 3,
            max_concurrent_requests: 50,
            timeout_seconds: 10,
            follow_redirects: true,
            user_agent: "ShadowProbe/0.1.0".to_string(),
            custom_headers: HashMap::new(),
            scan_types: vec![
                VulnerabilityType::SQLInjection,
                VulnerabilityType::XSS,
                VulnerabilityType::SSRF,
            ],
            enable_ai_analysis: true,
            aggressive_mode: false,
        }
    }
}
