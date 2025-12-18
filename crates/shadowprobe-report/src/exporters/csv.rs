use shadowprobe_core::{Result, ScanResult, ShadowProbeError};
use std::fs::File;
use std::io::Write;

pub struct CsvExporter;

impl CsvExporter {
    pub fn new() -> Self {
        Self
    }

    pub fn export(&self, scan_result: &ScanResult, path: &str) -> Result<()> {
        let mut file = File::create(path)?;

        // Write header
        writeln!(
            file,
            "ID,Type,Severity,URL,Method,Parameter,Payload,Description,CVSS Score,Timestamp"
        )?;

        // Write vulnerabilities
        for vuln in &scan_result.vulnerabilities {
            let param = vuln.parameter.as_ref().map(|s| s.as_str()).unwrap_or("");
            let score = vuln.ai_confidence.map(|c| (c * 10.0).to_string()).unwrap_or_else(|| "N/A".to_string());

            writeln!(
                file,
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"",
                self.escape_csv(&vuln.id),
                self.escape_csv(vuln.vuln_type.as_str()),
                vuln.severity.as_str(),
                self.escape_csv(&vuln.url),
                vuln.method.as_str(),
                self.escape_csv(param),
                self.escape_csv(&vuln.payload),
                self.escape_csv(&vuln.description),
                score,
                vuln.timestamp.to_rfc3339()
            )?;
        }

        Ok(())
    }

    fn escape_csv(&self, s: &str) -> String {
        s.replace('"', "\"\"")
    }
}

impl Default for CsvExporter {
    fn default() -> Self {
        Self::new()
    }
}
