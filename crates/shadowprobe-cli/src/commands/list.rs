use colored::*;

pub fn run() {
    println!("\n{}", "Available Vulnerability Scanners:".bright_cyan().bold());
    println!("{}", "─".repeat(50).bright_black());

    let scanners = vec![
        ("SQL Injection", "Detects SQL injection vulnerabilities using error-based, boolean-based, and time-based techniques"),
        ("Cross-Site Scripting (XSS)", "Identifies reflected, stored, and DOM-based XSS vulnerabilities"),
        ("Server-Side Request Forgery (SSRF)", "Detects SSRF vulnerabilities including cloud metadata access"),
        ("Local File Inclusion (LFI)", "Finds path traversal and local file inclusion vulnerabilities"),
        ("Server-Side Template Injection (SSTI)", "Detects template injection in various engines (Jinja2, Twig, etc.)"),
        ("Command Injection", "Identifies OS command injection vulnerabilities"),
        ("Open Redirect", "Finds unvalidated redirect vulnerabilities"),
    ];

    for (i, (name, desc)) in scanners.iter().enumerate() {
        println!(
            "\n{}. {}",
            (i + 1).to_string().bright_yellow(),
            name.bright_white().bold()
        );
        println!("   {}", desc.bright_black());
    }

    println!("\n{}", "─".repeat(50).bright_black());
    println!(
        "\n{} {}",
        "ℹ️".bright_blue(),
        "All scanners include AI-powered analysis for severity assessment and exploit generation".bright_white()
    );
    println!();
}
