use shadowprobe_core::{ScanConfig, ScanResult, ScanStatus};
use shadowprobe_scanner::{Crawler, HttpClient, ScannerEngine};
use shadowprobe_ai::VulnerabilityAnalyzer;
use shadowprobe_report::{ConsoleReporter, HtmlReporter, JsonReporter, Reporter};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;
use tokio::time::Instant;
use tracing::{info, warn};
use uuid::Uuid;
use chrono::Utc;

pub async fn run(
    url: String,
    depth: usize,
    concurrent: usize,
    timeout: u64,
    enable_ai: bool,
    _aggressive: bool,
    output: Option<String>,
    html: Option<String>,
    user_agent: String,
) -> anyhow::Result<()> {
    println!(
        "\n{} {}",
        "→".bright_cyan().bold(),
        format!("Starting scan of: {}", url).bright_white()
    );

    let start_time = Instant::now();
    let scan_start = Utc::now();

    // Create HTTP client
    info!("Initializing HTTP client...");
    let client = HttpClient::new(timeout, &user_agent, 5)?;

    // Phase 1: Crawling
    println!(
        "\n{} {}",
        "🕷️".bright_yellow(),
        "Phase 1: Web Crawling".bright_white().bold()
    );

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message("Discovering endpoints...");

    let crawler = Crawler::new(client.clone(), &url, depth, concurrent)?;
    let discovered_urls = crawler.crawl().await?;

    pb.finish_with_message(format!(
        "✓ Discovered {} unique endpoints",
        discovered_urls.len()
    ));

    // Phase 2: Vulnerability Scanning
    println!(
        "\n{} {}",
        "🔍".bright_yellow(),
        "Phase 2: Vulnerability Scanning".bright_white().bold()
    );

    let scanner = ScannerEngine::new();
    let pb = ProgressBar::new(discovered_urls.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );

    let mut all_vulns = Vec::new();
    let mut total_requests = 0u64;

    for (i, endpoint) in discovered_urls.iter().enumerate() {
        pb.set_message(format!("Scanning: {}", endpoint));

        match scanner.scan_url(&client, endpoint).await {
            Ok(vulns) => {
                total_requests += 1;
                all_vulns.extend(vulns);
            }
            Err(e) => {
                warn!("Error scanning {}: {}", endpoint, e);
            }
        }

        pb.set_position(i as u64 + 1);
    }

    pb.finish_with_message(format!(
        "✓ Scanned {} endpoints, found {} potential vulnerabilities",
        discovered_urls.len(),
        all_vulns.len()
    ));

    // Phase 3: AI Analysis
    if enable_ai && !all_vulns.is_empty() {
        println!(
            "\n{} {}",
            "🤖".bright_yellow(),
            "Phase 3: AI-Powered Analysis".bright_white().bold()
        );

        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.magenta} {msg}")
                .unwrap(),
        );
        pb.set_message("Analyzing vulnerabilities with AI...");

        let analyzer = VulnerabilityAnalyzer::new(true);
        analyzer.enhance_vulnerabilities(&mut all_vulns);
        analyzer.prioritize_vulnerabilities(&mut all_vulns);

        let filtered_count = all_vulns.len();
        all_vulns = analyzer.filter_false_positives(all_vulns);

        pb.finish_with_message(format!(
            "✓ AI analysis complete ({} likely false positives filtered)",
            filtered_count - all_vulns.len()
        ));
    }

    let elapsed = start_time.elapsed();

    // Create scan result
    let scan_result = ScanResult {
        target_url: url.clone(),
        scan_id: Uuid::new_v4().to_string(),
        start_time: scan_start,
        end_time: Some(Utc::now()),
        vulnerabilities: all_vulns.clone(),
        endpoints_discovered: discovered_urls.clone(),
        total_requests,
        status: ScanStatus::Completed,
    };

    // Display results
    println!(
        "\n{} {}",
        "📊".bright_yellow(),
        "Scan Complete".bright_white().bold()
    );

    let console_reporter = ConsoleReporter::new(true);
    let report = console_reporter.generate(&scan_result)?;
    print!("{}", report);

    println!(
        "{}",
        format!("⏱️  Total scan time: {:.2}s", elapsed.as_secs_f64())
            .bright_black()
    );

    // Save reports
    if let Some(json_path) = output {
        let json_reporter = JsonReporter::new();
        json_reporter.save_to_file(&scan_result, &json_path)?;
        println!(
            "{}",
            format!("💾 JSON report saved to: {}", json_path).bright_green()
        );
    }

    if let Some(html_path) = html {
        let html_reporter = HtmlReporter::new();
        html_reporter.save_to_file(&scan_result, &html_path)?;
        println!(
            "{}",
            format!("📄 HTML report saved to: {}", html_path).bright_green()
        );
    }

    // Summary
    if !all_vulns.is_empty() {
        println!(
            "\n{}",
            "⚠️  Security issues detected! Review the findings above.".bright_red().bold()
        );
    } else {
        println!(
            "\n{}",
            "✅ No vulnerabilities detected!".bright_green().bold()
        );
    }

    Ok(())
}
