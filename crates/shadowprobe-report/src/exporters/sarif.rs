use shadowprobe_core::{Result, ScanResult, Severity, ShadowProbeError};
use serde_json::{json, Value};
use std::fs;

/// SARIF (Static Analysis Results Interchange Format) exporter
/// Compatible with GitHub Security, VS Code, and other tools
pub struct SarifExporter;

impl SarifExporter {
    pub fn new() -> Self {
        Self
    }

    pub fn export(&self, scan_result: &ScanResult, path: &str) -> Result<()> {
        let sarif = self.generate_sarif(scan_result);
        let json_str = serde_json::to_string_pretty(&sarif)
            .map_err(|e| ShadowProbeError::SerializationError(e.to_string()))?;
        fs::write(path, json_str)?;
        Ok(())
    }

    fn generate_sarif(&self, scan_result: &ScanResult) -> Value {
        let rules: Vec<Value> = scan_result
            .vulnerabilities
            .iter()
            .map(|v| {
                json!({
                    "id": v.id,
                    "name": v.vuln_type.as_str(),
                    "shortDescription": {
                        "text": v.vuln_type.as_str()
                    },
                    "fullDescription": {
                        "text": v.description
                    },
                    "help": {
                        "text": v.remediation
                    },
                    "defaultConfiguration": {
                        "level": self.severity_to_sarif_level(&v.severity)
                    }
                })
            })
            .collect();

        let results: Vec<Value> = scan_result
            .vulnerabilities
            .iter()
            .map(|v| {
                json!({
                    "ruleId": v.id,
                    "level": self.severity_to_sarif_level(&v.severity),
                    "message": {
                        "text": v.description
                    },
                    "locations": [{
                        "physicalLocation": {
                            "artifactLocation": {
                                "uri": v.url
                            }
                        }
                    }],
                    "properties": {
                        "method": v.method.as_str(),
                        "parameter": v.parameter.as_ref().unwrap_or(&String::new()),
                        "payload": v.payload,
                        "evidence": v.evidence,
                        "ai_confidence": v.ai_confidence
                    }
                })
            })
            .collect();

        json!({
            "version": "2.1.0",
            "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
            "runs": [{
                "tool": {
                    "driver": {
                        "name": "ShadowProbe",
                        "version": "0.1.0",
                        "informationUri": "https://github.com/yourusername/shadowprobe",
                        "rules": rules
                    }
                },
                "results": results
            }]
        })
    }

    fn severity_to_sarif_level(&self, severity: &Severity) -> &str {
        match severity {
            Severity::Critical | Severity::High => "error",
            Severity::Medium => "warning",
            Severity::Low => "note",
            Severity::Info => "none",
        }
    }
}

impl Default for SarifExporter {
    fn default() -> Self {
        Self::new()
    }
}
