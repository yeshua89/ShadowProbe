pub mod json;
pub mod html;
pub mod console;
pub mod exporters;

use shadowprobe_core::{Result, ScanResult};

pub trait Reporter {
    fn generate(&self, scan_result: &ScanResult) -> Result<String>;
    fn save_to_file(&self, scan_result: &ScanResult, path: &str) -> Result<()>;
}

pub use json::JsonReporter;
pub use html::HtmlReporter;
pub use console::ConsoleReporter;
pub use exporters::*;
