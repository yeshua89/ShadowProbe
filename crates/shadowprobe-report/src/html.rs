use crate::Reporter;
use shadowprobe_core::{Result, ScanResult, Severity};
use std::fs;

pub struct HtmlReporter;

impl HtmlReporter {
    pub fn new() -> Self {
        Self
    }

    fn severity_color(severity: &Severity) -> &str {
        match severity {
            Severity::Critical => "#9c27b0",
            Severity::High => "#f44336",
            Severity::Medium => "#ff9800",
            Severity::Low => "#2196f3",
            Severity::Info => "#9e9e9e",
        }
    }
}

impl Reporter for HtmlReporter {
    fn generate(&self, scan_result: &ScanResult) -> Result<String> {
        let mut html = String::from(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ShadowProbe - Scan Report</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            background: #0a0a0a;
            color: #e0e0e0;
            padding: 20px;
        }
        .container { max-width: 1200px; margin: 0 auto; }
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            padding: 30px;
            border-radius: 10px;
            margin-bottom: 30px;
            box-shadow: 0 4px 6px rgba(0,0,0,0.3);
        }
        .header h1 {
            font-size: 2.5em;
            margin-bottom: 10px;
            color: white;
        }
        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        .stat-card {
            background: #1a1a1a;
            padding: 20px;
            border-radius: 8px;
            border: 1px solid #333;
        }
        .stat-card h3 { color: #888; font-size: 0.9em; margin-bottom: 10px; }
        .stat-card .value { font-size: 2em; font-weight: bold; color: #667eea; }
        .vulnerabilities { margin-top: 30px; }
        .vuln-card {
            background: #1a1a1a;
            border-left: 4px solid;
            padding: 20px;
            margin-bottom: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.2);
        }
        .vuln-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 15px;
        }
        .vuln-title { font-size: 1.3em; font-weight: bold; }
        .severity-badge {
            padding: 5px 15px;
            border-radius: 20px;
            font-size: 0.8em;
            font-weight: bold;
            color: white;
        }
        .vuln-details { margin-top: 15px; }
        .detail-row {
            margin: 10px 0;
            padding: 10px;
            background: #0a0a0a;
            border-radius: 4px;
        }
        .detail-label { color: #888; font-size: 0.9em; margin-bottom: 5px; }
        .detail-value {
            color: #e0e0e0;
            font-family: 'Courier New', monospace;
            word-break: break-all;
        }
        .code-block {
            background: #0a0a0a;
            padding: 15px;
            border-radius: 4px;
            overflow-x: auto;
            margin: 10px 0;
        }
        .code-block code {
            color: #4caf50;
            font-family: 'Courier New', monospace;
        }
        .ai-analysis {
            background: #1a237e;
            padding: 15px;
            border-radius: 4px;
            margin-top: 10px;
            border-left: 3px solid #2196f3;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔍 ShadowProbe</h1>
            <p style="color: rgba(255,255,255,0.9);">Vulnerability Scan Report</p>
        </div>
"#,
        );

        // Stats
        let critical_count = scan_result
            .vulnerabilities
            .iter()
            .filter(|v| matches!(v.severity, Severity::Critical))
            .count();
        let high_count = scan_result
            .vulnerabilities
            .iter()
            .filter(|v| matches!(v.severity, Severity::High))
            .count();
        let medium_count = scan_result
            .vulnerabilities
            .iter()
            .filter(|v| matches!(v.severity, Severity::Medium))
            .count();
        let low_count = scan_result
            .vulnerabilities
            .iter()
            .filter(|v| matches!(v.severity, Severity::Low))
            .count();

        html.push_str(&format!(
            r#"
        <div class="stats">
            <div class="stat-card">
                <h3>Target</h3>
                <div class="value" style="font-size: 1.2em; color: #e0e0e0;">{}</div>
            </div>
            <div class="stat-card">
                <h3>Total Vulnerabilities</h3>
                <div class="value">{}</div>
            </div>
            <div class="stat-card">
                <h3>Critical / High</h3>
                <div class="value" style="color: #f44336;">{} / {}</div>
            </div>
            <div class="stat-card">
                <h3>Total Requests</h3>
                <div class="value">{}</div>
            </div>
        </div>
"#,
            scan_result.target_url,
            scan_result.vulnerabilities.len(),
            critical_count,
            high_count,
            scan_result.total_requests
        ));

        // Vulnerabilities
        html.push_str(r#"<div class="vulnerabilities"><h2 style="margin-bottom: 20px;">Discovered Vulnerabilities</h2>"#);

        for vuln in &scan_result.vulnerabilities {
            let color = Self::severity_color(&vuln.severity);
            html.push_str(&format!(
                r#"
        <div class="vuln-card" style="border-left-color: {};">
            <div class="vuln-header">
                <div class="vuln-title">{}</div>
                <span class="severity-badge" style="background-color: {};">{}</span>
            </div>
            <div class="vuln-details">
                <div class="detail-row">
                    <div class="detail-label">URL</div>
                    <div class="detail-value">{}</div>
                </div>
                <div class="detail-row">
                    <div class="detail-label">Method</div>
                    <div class="detail-value">{}</div>
                </div>
"#,
                color,
                vuln.vuln_type.as_str(),
                color,
                vuln.severity.as_str(),
                vuln.url,
                vuln.method.as_str()
            ));

            if let Some(param) = &vuln.parameter {
                html.push_str(&format!(
                    r#"
                <div class="detail-row">
                    <div class="detail-label">Parameter</div>
                    <div class="detail-value">{}</div>
                </div>
"#,
                    param
                ));
            }

            html.push_str(&format!(
                r#"
                <div class="detail-row">
                    <div class="detail-label">Payload</div>
                    <div class="detail-value">{}</div>
                </div>
                <div class="detail-row">
                    <div class="detail-label">Description</div>
                    <div class="detail-value">{}</div>
                </div>
"#,
                html_escape::encode_text(&vuln.payload),
                html_escape::encode_text(&vuln.description)
            ));

            if let Some(poc) = &vuln.poc {
                html.push_str(&format!(
                    r#"
                <div class="code-block">
                    <div class="detail-label">Proof of Concept</div>
                    <code>{}</code>
                </div>
"#,
                    html_escape::encode_text(poc)
                ));
            }

            if let Some(ai_analysis) = &vuln.ai_analysis {
                html.push_str(&format!(
                    r#"
                <div class="ai-analysis">
                    <strong>🤖 AI Analysis</strong>
                    <pre style="margin-top: 10px; white-space: pre-wrap; color: #e0e0e0;">{}</pre>
                </div>
"#,
                    html_escape::encode_text(ai_analysis)
                ));
            }

            html.push_str("</div></div>");
        }

        html.push_str(
            r#"
        </div>
    </div>
</body>
</html>
"#,
        );

        Ok(html)
    }

    fn save_to_file(&self, scan_result: &ScanResult, path: &str) -> Result<()> {
        let html = self.generate(scan_result)?;
        fs::write(path, html)?;
        Ok(())
    }
}

impl Default for HtmlReporter {
    fn default() -> Self {
        Self::new()
    }
}
