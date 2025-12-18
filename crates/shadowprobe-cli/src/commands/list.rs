use colored::*;

pub fn run() {
    println!("\n{}", "🔍 Available Vulnerability Scanners".bright_cyan().bold());
    println!("{}", "═".repeat(80).bright_black());

    let scanners = vec![
        ("SQL Injection", "14 payloads", "Detects SQL injection using error-based, boolean, union, time-based, and stacked queries. Supports MySQL, PostgreSQL, MSSQL, Oracle."),
        ("Cross-Site Scripting (XSS)", "5 payloads", "Identifies reflected, stored, and DOM-based XSS. Includes polyglots and context-aware detection."),
        ("Server-Side Request Forgery (SSRF)", "5 payloads", "Detects SSRF including AWS/GCP metadata exploitation, localhost access, and file protocol abuse."),
        ("Local File Inclusion (LFI)", "4 payloads", "Finds path traversal and LFI with encoding bypass techniques. Tests for /etc/passwd and Windows files."),
        ("Server-Side Template Injection (SSTI)", "4 payloads", "Detects SSTI in Jinja2, Twig, Freemarker, and ERB. Critical severity with RCE potential."),
        ("Command Injection", "5 payloads", "Identifies OS command injection with timing analysis. Tests multiple shell metacharacters."),
        ("XML External Entity (XXE)", "5 payloads", "Detects XXE in XML parsers. Includes classic, parameter entity, and blind XXE techniques."),
        ("CORS Misconfiguration", "4 tests", "Identifies dangerous CORS policies including wildcard origins and credential exposure."),
        ("Open Redirect", "7 payloads", "Finds unvalidated redirects with multiple bypass techniques (protocol-relative, backslash, @-sign)."),
    ];

    for (i, (name, payloads, desc)) in scanners.iter().enumerate() {
        println!();
        println!(
            "{}. {} {}",
            (i + 1).to_string().bright_yellow().bold(),
            name.bright_white().bold(),
            format!("[{}]", payloads).bright_black()
        );
        println!("   {}", desc.bright_white());
    }

    println!();
    println!("{}", "─".repeat(80).bright_black());
    println!();
    println!("{}", "🎯 Features:".bright_green().bold());
    println!("   • AI-powered severity assessment and prioritization");
    println!("   • Automatic PoC generation for confirmed vulnerabilities");
    println!("   • WAF evasion techniques (encoding, obfuscation)");
    println!("   • Adaptive rate limiting to respect target servers");
    println!("   • Request caching to prevent duplicate scans");
    println!();
    println!("{}", "📊 Total: 57+ unique payloads across 9 scanner types".bright_cyan());
    println!();
}
