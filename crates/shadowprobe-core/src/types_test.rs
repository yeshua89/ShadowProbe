#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn test_http_method_as_str() {
        assert_eq!(HttpMethod::GET.as_str(), "GET");
        assert_eq!(HttpMethod::POST.as_str(), "POST");
    }

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::High);
        assert!(Severity::High > Severity::Medium);
        assert!(Severity::Medium > Severity::Low);
        assert!(Severity::Low > Severity::Info);
    }

    #[test]
    fn test_severity_color_code() {
        let critical = Severity::Critical;
        assert!(!critical.color_code().is_empty());
    }

    #[test]
    fn test_vulnerability_type_as_str() {
        let sqli = VulnerabilityType::SQLInjection;
        assert_eq!(sqli.as_str(), "SQL Injection");

        let xss = VulnerabilityType::XSS;
        assert_eq!(xss.as_str(), "Cross-Site Scripting (XSS)");
    }

    #[test]
    fn test_scan_config_default() {
        let config = ScanConfig::default();
        assert_eq!(config.max_depth, 3);
        assert_eq!(config.max_concurrent_requests, 50);
        assert!(config.enable_ai_analysis);
    }

    #[test]
    fn test_scan_status() {
        let status = ScanStatus::Running;
        assert_eq!(status, ScanStatus::Running);
        assert_ne!(status, ScanStatus::Completed);
    }
}
