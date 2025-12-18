use crate::Reporter;
use shadowprobe_core::{Result, ScanResult, ShadowProbeError};
use serde_json;
use std::fs;

pub struct JsonReporter;

impl JsonReporter {
    pub fn new() -> Self {
        Self
    }
}

impl Reporter for JsonReporter {
    fn generate(&self, scan_result: &ScanResult) -> Result<String> {
        serde_json::to_string_pretty(scan_result)
            .map_err(|e| ShadowProbeError::SerializationError(e.to_string()))
    }

    fn save_to_file(&self, scan_result: &ScanResult, path: &str) -> Result<()> {
        let json = self.generate(scan_result)?;
        fs::write(path, json)?;
        Ok(())
    }
}

impl Default for JsonReporter {
    fn default() -> Self {
        Self::new()
    }
}
