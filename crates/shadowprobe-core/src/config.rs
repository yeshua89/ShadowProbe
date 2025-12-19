use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::{Result, ShadowProbeError};

/// Scan profile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProfile {
    pub name: String,
    pub description: String,
    pub max_depth: usize,
    pub max_concurrent_requests: usize,
    pub timeout_seconds: u64,
    pub follow_redirects: bool,
    pub user_agent: String,
    pub custom_headers: HashMap<String, String>,
    pub enabled_scanners: Vec<String>,
    pub enable_ai_analysis: bool,
    pub aggressive_mode: bool,
    pub use_evasion_techniques: bool,
    pub rate_limit_ms: Option<u64>,
}

impl ScanProfile {
    /// Fast scan profile - quick reconnaissance
    pub fn fast() -> Self {
        Self {
            name: "fast".to_string(),
            description: "Quick scan with minimal payloads".to_string(),
            max_depth: 2,
            max_concurrent_requests: 100,
            timeout_seconds: 5,
            follow_redirects: true,
            user_agent: "ShadowProbe/0.1.0 (Fast)".to_string(),
            custom_headers: HashMap::new(),
            enabled_scanners: vec!["xss".to_string(), "sqli".to_string()],
            enable_ai_analysis: false,
            aggressive_mode: false,
            use_evasion_techniques: false,
            rate_limit_ms: Some(100),
        }
    }

    /// Balanced scan profile - default
    pub fn balanced() -> Self {
        Self {
            name: "balanced".to_string(),
            description: "Balanced scan with moderate coverage".to_string(),
            max_depth: 3,
            max_concurrent_requests: 50,
            timeout_seconds: 10,
            follow_redirects: true,
            user_agent: "ShadowProbe/0.1.0".to_string(),
            custom_headers: HashMap::new(),
            enabled_scanners: vec![
                "sqli".to_string(),
                "xss".to_string(),
                "ssrf".to_string(),
                "lfi".to_string(),
            ],
            enable_ai_analysis: true,
            aggressive_mode: false,
            use_evasion_techniques: false,
            rate_limit_ms: None,
        }
    }

    /// Deep scan profile - comprehensive testing
    pub fn deep() -> Self {
        Self {
            name: "deep".to_string(),
            description: "Comprehensive scan with all techniques".to_string(),
            max_depth: 5,
            max_concurrent_requests: 30,
            timeout_seconds: 15,
            follow_redirects: true,
            user_agent: "ShadowProbe/0.1.0 (Deep)".to_string(),
            custom_headers: HashMap::new(),
            enabled_scanners: vec![
                "sqli".to_string(),
                "xss".to_string(),
                "ssrf".to_string(),
                "lfi".to_string(),
                "ssti".to_string(),
            ],
            enable_ai_analysis: true,
            aggressive_mode: true,
            use_evasion_techniques: true,
            rate_limit_ms: None,
        }
    }

    /// Stealth scan profile - evade detection
    pub fn stealth() -> Self {
        Self {
            name: "stealth".to_string(),
            description: "Slow and stealthy scan to evade detection".to_string(),
            max_depth: 3,
            max_concurrent_requests: 5,
            timeout_seconds: 20,
            follow_redirects: true,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            custom_headers: {
                let mut headers = HashMap::new();
                headers.insert("Accept".to_string(), "text/html,application/xhtml+xml".to_string());
                headers.insert("Accept-Language".to_string(), "en-US,en;q=0.9".to_string());
                headers
            },
            enabled_scanners: vec![
                "sqli".to_string(),
                "xss".to_string(),
            ],
            enable_ai_analysis: true,
            aggressive_mode: false,
            use_evasion_techniques: true,
            rate_limit_ms: Some(2000),
        }
    }

    /// Load profile from YAML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        serde_yaml::from_str(&content)
            .map_err(|e| ShadowProbeError::ParseError(e.to_string()))
    }

    /// Save profile to YAML file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let yaml = serde_yaml::to_string(self)
            .map_err(|e| ShadowProbeError::SerializationError(e.to_string()))?;
        fs::write(path, yaml)?;
        Ok(())
    }

    /// Get profile by name
    pub fn by_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "fast" => Some(Self::fast()),
            "balanced" => Some(Self::balanced()),
            "deep" => Some(Self::deep()),
            "stealth" => Some(Self::stealth()),
            _ => None,
        }
    }
}

impl Default for ScanProfile {
    fn default() -> Self {
        Self::balanced()
    }
}

/// Global configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub default_profile: String,
    pub output_directory: String,
    pub enable_logging: bool,
    pub log_level: String,
    pub api_keys: HashMap<String, String>,
    pub custom_wordlists: HashMap<String, String>,
}

impl Config {
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        if path.as_ref().exists() {
            Self::from_file(path).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        serde_yaml::from_str(&content)
            .map_err(|e| ShadowProbeError::ParseError(e.to_string()))
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let yaml = serde_yaml::to_string(self)
            .map_err(|e| ShadowProbeError::SerializationError(e.to_string()))?;
        fs::write(path, yaml)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_profile: "balanced".to_string(),
            output_directory: "./output".to_string(),
            enable_logging: true,
            log_level: "info".to_string(),
            api_keys: HashMap::new(),
            custom_wordlists: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiles() {
        let fast = ScanProfile::fast();
        assert_eq!(fast.max_depth, 2);

        let deep = ScanProfile::deep();
        assert!(deep.aggressive_mode);
    }

    #[test]
    fn test_profile_by_name() {
        let profile = ScanProfile::by_name("stealth");
        assert!(profile.is_some());
        assert_eq!(profile.unwrap().name, "stealth");
    }
}
