use crate::model::VulnAnalysisModel;
use shadowprobe_core::{Severity, Vulnerability};
use tracing::info;

pub struct VulnerabilityAnalyzer {
    model: VulnAnalysisModel,
}

impl VulnerabilityAnalyzer {
    pub fn new(enable_ai: bool) -> Self {
        Self {
            model: VulnAnalysisModel::new(enable_ai),
        }
    }

    /// Enhance vulnerabilities with AI analysis
    pub fn enhance_vulnerabilities(&self, vulns: &mut Vec<Vulnerability>) {
        info!("Enhancing {} vulnerabilities with AI analysis", vulns.len());

        for vuln in vulns.iter_mut() {
            let (confidence, analysis) = self.model.analyze_vulnerability(
                vuln.vuln_type.as_str(),
                &vuln.evidence,
                &vuln.evidence,
            );

            vuln.ai_confidence = Some(confidence);
            vuln.ai_analysis = Some(analysis);

            // Adjust severity based on confidence
            if confidence < 0.4 {
                vuln.severity = self.downgrade_severity(&vuln.severity);
            }

            // Generate PoC if confidence is high
            if confidence >= 0.7 && vuln.poc.is_none() {
                vuln.poc = self.model.generate_poc(
                    vuln.vuln_type.as_str(),
                    &vuln.url,
                    &vuln.payload,
                );
            }
        }
    }

    /// Prioritize vulnerabilities by exploitability
    pub fn prioritize_vulnerabilities(&self, vulns: &mut Vec<Vulnerability>) {
        vulns.sort_by(|a, b| {
            let a_score = self.calculate_priority_score(a);
            let b_score = self.calculate_priority_score(b);
            b_score.partial_cmp(&a_score).unwrap()
        });
    }

    fn calculate_priority_score(&self, vuln: &Vulnerability) -> f32 {
        let severity_score = match vuln.severity {
            Severity::Critical => 5.0,
            Severity::High => 4.0,
            Severity::Medium => 3.0,
            Severity::Low => 2.0,
            Severity::Info => 1.0,
        };

        let confidence_score = vuln.ai_confidence.unwrap_or(0.5);

        severity_score * confidence_score
    }

    fn downgrade_severity(&self, severity: &Severity) -> Severity {
        match severity {
            Severity::Critical => Severity::High,
            Severity::High => Severity::Medium,
            Severity::Medium => Severity::Low,
            Severity::Low => Severity::Info,
            Severity::Info => Severity::Info,
        }
    }

    /// Filter out likely false positives
    pub fn filter_false_positives(&self, vulns: Vec<Vulnerability>) -> Vec<Vulnerability> {
        vulns
            .into_iter()
            .filter(|v| {
                if let Some(confidence) = v.ai_confidence {
                    confidence >= 0.3
                } else {
                    true
                }
            })
            .collect()
    }
}

impl Default for VulnerabilityAnalyzer {
    fn default() -> Self {
        Self::new(true)
    }
}
