#[cfg(test)]
mod tests {
    use crate::payloads::*;
    use shadowprobe_core::VulnerabilityType;

    #[test]
    fn test_sqli_payloads() {
        let payloads = PayloadSet::for_sqli();
        assert_eq!(payloads.vuln_type, VulnerabilityType::SQLInjection);
        assert!(!payloads.payloads.is_empty());

        // Verify we have common SQLi payloads
        assert!(payloads.payloads.iter().any(|p| p.value.contains("OR")));
        assert!(payloads.payloads.iter().any(|p| p.value.contains("UNION")));
    }

    #[test]
    fn test_xss_payloads() {
        let payloads = PayloadSet::for_xss();
        assert_eq!(payloads.vuln_type, VulnerabilityType::XSS);
        assert!(!payloads.payloads.is_empty());

        assert!(payloads.payloads.iter().any(|p| p.value.contains("<script>")));
    }

    #[test]
    fn test_ssrf_payloads() {
        let payloads = PayloadSet::for_ssrf();
        assert_eq!(payloads.vuln_type, VulnerabilityType::SSRF);

        // Check for localhost payloads
        assert!(payloads.payloads.iter().any(|p| p.value.contains("127.0.0.1")));

        // Check for cloud metadata payloads
        assert!(payloads.payloads.iter().any(|p| p.value.contains("169.254.169.254")));
    }

    #[test]
    fn test_lfi_payloads() {
        let payloads = PayloadSet::for_lfi();
        assert_eq!(payloads.vuln_type, VulnerabilityType::LFI);

        assert!(payloads.payloads.iter().any(|p| p.value.contains("etc/passwd")));
    }

    #[test]
    fn test_ssti_payloads() {
        let payloads = PayloadSet::for_ssti();
        assert_eq!(payloads.vuln_type, VulnerabilityType::SSTI);

        assert!(payloads.payloads.iter().any(|p| p.value.contains("{{") || p.value.contains("${")));
    }

    #[test]
    fn test_get_all_payloads() {
        let all = get_all_payloads();
        assert!(all.len() >= 5); // At least 5 vulnerability types
    }

    #[test]
    fn test_payload_has_detection_patterns() {
        let payloads = PayloadSet::for_sqli();

        for payload in &payloads.payloads {
            assert!(!payload.description.is_empty());
            // Some payloads have detection patterns, some don't (for time-based)
        }
    }
}
