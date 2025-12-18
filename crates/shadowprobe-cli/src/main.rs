mod commands;

use clap::{Parser, Subcommand};
use colored::*;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(name = "shadowprobe")]
#[command(author = "ShadowProbe Contributors")]
#[command(version = "0.1.0")]
#[command(about = "Ultra-fast web vulnerability scanner with AI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a target URL for vulnerabilities
    Scan {
        /// Target URL to scan
        #[arg(short, long)]
        url: String,

        /// Maximum crawl depth
        #[arg(short, long, default_value = "3")]
        depth: usize,

        /// Maximum concurrent requests
        #[arg(short, long, default_value = "50")]
        concurrent: usize,

        /// Request timeout in seconds
        #[arg(short, long, default_value = "10")]
        timeout: u64,

        /// Disable AI analysis
        #[arg(long)]
        no_ai: bool,

        /// Enable aggressive scanning (more payloads, higher risk)
        #[arg(short, long)]
        aggressive: bool,

        /// Output file path (JSON)
        #[arg(short, long)]
        output: Option<String>,

        /// Generate HTML report
        #[arg(long)]
        html: Option<String>,

        /// Custom User-Agent
        #[arg(long, default_value = "ShadowProbe/0.1.0")]
        user_agent: String,
    },

    /// List available vulnerability scanners
    List,

    /// Show version information
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Setup tracing
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new(log_level))
        .init();

    // Disable colors if requested
    if cli.no_color {
        colored::control::set_override(false);
    }

    // Print banner
    print_banner();

    match cli.command {
        Commands::Scan {
            url,
            depth,
            concurrent,
            timeout,
            no_ai,
            aggressive,
            output,
            html,
            user_agent,
        } => {
            commands::scan::run(
                url,
                depth,
                concurrent,
                timeout,
                !no_ai,
                aggressive,
                output,
                html,
                user_agent,
            )
            .await?;
        }
        Commands::List => {
            commands::list::run();
        }
        Commands::Version => {
            commands::version::run();
        }
    }

    Ok(())
}

fn print_banner() {
    let banner = r#"
    ███████╗██╗  ██╗ █████╗ ██████╗  ██████╗ ██╗    ██╗
    ██╔════╝██║  ██║██╔══██╗██╔══██╗██╔═══██╗██║    ██║
    ███████╗███████║███████║██║  ██║██║   ██║██║ █╗ ██║
    ╚════██║██╔══██║██╔══██║██║  ██║██║   ██║██║███╗██║
    ███████║██║  ██║██║  ██║██████╔╝╚██████╔╝╚███╔███╔╝
    ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝  ╚═════╝  ╚══╝╚══╝
    ██████╗ ██████╗  ██████╗ ██████╗ ███████╗
    ██╔══██╗██╔══██╗██╔═══██╗██╔══██╗██╔════╝
    ██████╔╝██████╔╝██║   ██║██████╔╝█████╗
    ██╔═══╝ ██╔══██╗██║   ██║██╔══██╗██╔══╝
    ██║     ██║  ██║╚██████╔╝██████╔╝███████╗
    ╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚═════╝ ╚══════╝
    "#;

    println!("{}", banner.bright_magenta().bold());
    println!(
        "{}",
        "    Ultra-fast Web Vulnerability Scanner with AI"
            .bright_cyan()
            .italic()
    );
    println!("{}", "    v0.1.0\n".bright_black());
}
