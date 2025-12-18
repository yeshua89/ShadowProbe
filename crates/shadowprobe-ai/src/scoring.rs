use shadowprobe_core::{Severity, Vulnerability, VulnerabilityType};

/// CVSS-like scoring system for vulnerabilities
pub struct VulnerabilityScorer;

impl VulnerabilityScorer {
    pub fn new() -> Self {
        Self
    }

    /// Calculate CVSS-like score (0.0 - 10.0)
    pub fn calculate_score(&self, vuln: &Vulnerability) -> f32 {
        let base_score = self.base_score(&vuln.vuln_type);
        let impact_score = self.impact_score(&vuln.severity);
        let exploitability = self.exploitability_score(vuln);

        // Weighted average
        (base_score * 0.3 + impact_score * 0.4 + exploitability * 0.3).min(10.0)
    }

    fn base_score(&self, vuln_type: &VulnerabilityType) -> f32 {
        match vuln_type {
            VulnerabilityType::SQLInjection => 9.0,
            VulnerabilityType::CommandInjection => 9.5,
            VulnerabilityType::SSTI => 9.0,
            VulnerabilityType::XXE => 8.5,
            VulnerabilityType::SSRF => 8.0,
            VulnerabilityType::XSS => 7.0,
            VulnerabilityType::LFI => 7.5,
            VulnerabilityType::RFI => 8.5,
            VulnerabilityType::CORS => 6.0,
            VulnerabilityType::OpenRedirect => 5.0,
            VulnerabilityType::CSRF => 6.5,
            VulnerabilityType::PathTraversal => 7.0,
            _ => 5.0,
        }
    }

    fn impact_score(&self, severity: &Severity) -> f32 {
        match severity {
            Severity::Critical => 10.0,
            Severity::High => 8.0,
            Severity::Medium => 5.0,
            Severity::Low => 3.0,
            Severity::Info => 1.0,
        }
    }

    fn exploitability_score(&self, vuln: &Vulnerability) -> f32 {
        let mut score = 5.0;

        // Has PoC = easier to exploit
        if vuln.poc.is_some() {
            score += 2.0;
        }

        // Strong evidence = more reliable
        if !vuln.evidence.is_empty() && vuln.evidence.len() > 50 {
            score += 1.5;
        }

        // AI confidence
        if let Some(confidence) = vuln.ai_confidence {
            score += confidence * 2.0;
        }

        score.min(10.0)
    }

    /// Get risk rating from score
    pub fn risk_rating(&self, score: f32) -> &str {
        match score {
            s if s >= 9.0 => "CRITICAL",
            s if s >= 7.0 => "HIGH",
            s if s >= 4.0 => "MEDIUM",
            s if s >= 1.0 => "LOW",
            _ => "INFO",
        }
    }

    /// Calculate exploitability percentage
    pub fn exploitability_percentage(&self, vuln: &Vulnerability) -> f32 {
        let score = self.exploitability_score(vuln);
        (score / 10.0 * 100.0).min(100.0)
    }

    /// Priority score for sorting (higher = more urgent)
    pub fn priority_score(&self, vuln: &Vulnerability) -> f32 {
        let cvss = self.calculate_score(vuln);
        let exploit = self.exploitability_percentage(vuln);

        // Combine CVSS and exploitability
        cvss * 0.6 + (exploit / 10.0) * 0.4
    }
}

impl Default for VulnerabilityScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use shadowprobe_core::HttpMethod;

    #[test]
    fn test_scoring() {
        let scorer = VulnerabilityScorer::new();

        let vuln = Vulnerability {
            id: "test".to_string(),
            vuln_type: VulnerabilityType::SQLInjection,
            severity: Severity::Critical,
            url: "http://test.com".to_string(),
            method: HttpMethod::GET,
            parameter: Some("id".to_string()),
            payload: "' OR 1=1--".to_string(),
            evidence: "SQL error in response".to_string(),
            description: "Test".to_string(),
            remediation: "Test".to_string(),
            poc: Some("curl test".to_string()),
            ai_confidence: Some(0.9),
            ai_analysis: None,
            timestamp: Utc::now(),
        };

        let score = scorer.calculate_score(&vuln);
        assert!(score > 8.0);
        assert_eq!(scorer.risk_rating(score), "CRITICAL");
    }
}
