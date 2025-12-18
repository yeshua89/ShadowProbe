use crate::Reporter;
use shadowprobe_core::{Result, ScanResult, Severity};

pub struct ConsoleReporter {
    colored: bool,
}

impl ConsoleReporter {
    pub fn new(colored: bool) -> Self {
        Self { colored }
    }

    fn colorize(&self, text: &str, color_code: &str) -> String {
        if self.colored {
            format!("{}{}\x1b[0m", color_code, text)
        } else {
            text.to_string()
        }
    }
}

impl Reporter for ConsoleReporter {
    fn generate(&self, scan_result: &ScanResult) -> Result<String> {
        let mut output = String::new();

        output.push_str(&self.colorize(
            "\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n",
            "\x1b[1;36m",
        ));
        output.push_str(&self.colorize("  🔍 ShadowProbe - Scan Results\n", "\x1b[1;35m"));
        output.push_str(&self.colorize(
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n",
            "\x1b[1;36m",
        ));

        output.push_str(&format!("Target: {}\n", scan_result.target_url));
        output.push_str(&format!("Scan ID: {}\n", scan_result.scan_id));
        output.push_str(&format!(
            "Total Vulnerabilities Found: {}\n",
            scan_result.vulnerabilities.len()
        ));
        output.push_str(&format!(
            "Endpoints Discovered: {}\n",
            scan_result.endpoints_discovered.len()
        ));
        output.push_str(&format!("Total Requests: {}\n\n", scan_result.total_requests));

        if scan_result.vulnerabilities.is_empty() {
            output.push_str(&self.colorize(
                "✓ No vulnerabilities detected!\n\n",
                "\x1b[1;32m",
            ));
        } else {
            output.push_str(&self.colorize(
                "⚠️  Vulnerabilities Detected:\n\n",
                "\x1b[1;33m",
            ));

            for (i, vuln) in scan_result.vulnerabilities.iter().enumerate() {
                let severity_str = self.colorize(
                    &format!("[{}]", vuln.severity.as_str()),
                    vuln.severity.color_code(),
                );

                output.push_str(&format!("{}. {} {}\n", i + 1, severity_str, vuln.vuln_type.as_str()));
                output.push_str(&format!("   URL: {}\n", vuln.url));
                output.push_str(&format!("   Method: {}\n", vuln.method.as_str()));

                if let Some(param) = &vuln.parameter {
                    output.push_str(&format!("   Parameter: {}\n", param));
                }

                output.push_str(&format!("   Payload: {}\n", vuln.payload));

                if let Some(confidence) = vuln.ai_confidence {
                    output.push_str(&format!("   AI Confidence: {:.1}%\n", confidence * 100.0));
                }

                output.push_str("\n");
            }
        }

        output.push_str(&self.colorize(
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n",
            "\x1b[1;36m",
        ));

        Ok(output)
    }

    fn save_to_file(&self, scan_result: &ScanResult, path: &str) -> Result<()> {
        let output = self.generate(scan_result)?;
        std::fs::write(path, output)?;
        Ok(())
    }
}

impl Default for ConsoleReporter {
    fn default() -> Self {
        Self::new(true)
    }
}
