use shadowprobe_core::{Result, ScanResult, Severity};
use std::fs::File;
use std::io::Write;

pub struct MarkdownExporter;

impl MarkdownExporter {
    pub fn new() -> Self {
        Self
    }

    pub fn export(&self, scan_result: &ScanResult, path: &str) -> Result<()> {
        let mut file = File::create(path)?;

        // Title
        writeln!(file, "# Security Scan Report")?;
        writeln!(file)?;
        writeln!(file, "**Target:** {}", scan_result.target_url)?;
        writeln!(file, "**Scan ID:** {}", scan_result.scan_id)?;
        writeln!(file, "**Date:** {}", scan_result.start_time.format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(file)?;

        // Summary
        writeln!(file, "## Summary")?;
        writeln!(file)?;
        let critical = scan_result.vulnerabilities.iter().filter(|v| matches!(v.severity, Severity::Critical)).count();
        let high = scan_result.vulnerabilities.iter().filter(|v| matches!(v.severity, Severity::High)).count();
        let medium = scan_result.vulnerabilities.iter().filter(|v| matches!(v.severity, Severity::Medium)).count();
        let low = scan_result.vulnerabilities.iter().filter(|v| matches!(v.severity, Severity::Low)).count();

        writeln!(file, "- **Total Vulnerabilities:** {}", scan_result.vulnerabilities.len())?;
        writeln!(file, "- **Critical:** {}", critical)?;
        writeln!(file, "- **High:** {}", high)?;
        writeln!(file, "- **Medium:** {}", medium)?;
        writeln!(file, "- **Low:** {}", low)?;
        writeln!(file, "- **Endpoints Scanned:** {}", scan_result.endpoints_discovered.len())?;
        writeln!(file, "- **Total Requests:** {}", scan_result.total_requests)?;
        writeln!(file)?;

        // Vulnerabilities
        if !scan_result.vulnerabilities.is_empty() {
            writeln!(file, "## Vulnerabilities")?;
            writeln!(file)?;

            for (i, vuln) in scan_result.vulnerabilities.iter().enumerate() {
                writeln!(file, "### {}. {} - {}", i + 1, vuln.vuln_type.as_str(), vuln.severity.as_str())?;
                writeln!(file)?;
                writeln!(file, "**URL:** `{}`", vuln.url)?;
                writeln!(file, "**Method:** {}", vuln.method.as_str())?;
                if let Some(param) = &vuln.parameter {
                    writeln!(file, "**Parameter:** `{}`", param)?;
                }
                writeln!(file, "**Payload:** `{}`", vuln.payload)?;
                writeln!(file)?;
                writeln!(file, "**Description:**")?;
                writeln!(file, "{}", vuln.description)?;
                writeln!(file)?;
                writeln!(file, "**Remediation:**")?;
                writeln!(file, "{}", vuln.remediation)?;
                writeln!(file)?;

                if let Some(poc) = &vuln.poc {
                    writeln!(file, "**Proof of Concept:**")?;
                    writeln!(file, "```bash")?;
                    writeln!(file, "{}", poc)?;
                    writeln!(file, "```")?;
                    writeln!(file)?;
                }

                if let Some(analysis) = &vuln.ai_analysis {
                    writeln!(file, "**AI Analysis:**")?;
                    writeln!(file, "```")?;
                    writeln!(file, "{}", analysis)?;
                    writeln!(file, "```")?;
                    writeln!(file)?;
                }

                writeln!(file, "---")?;
                writeln!(file)?;
            }
        }

        Ok(())
    }
}

impl Default for MarkdownExporter {
    fn default() -> Self {
        Self::new()
    }
}
