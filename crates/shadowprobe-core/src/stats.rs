use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStatistics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_bytes_transferred: u64,
    pub average_response_time: Duration,
    pub fastest_response: Duration,
    pub slowest_response: Duration,
    pub requests_per_second: f64,
    pub endpoints_discovered: usize,
    pub vulnerabilities_found: usize,
    pub vulnerabilities_by_type: HashMap<String, usize>,
    pub vulnerabilities_by_severity: HashMap<String, usize>,
    pub false_positives_filtered: usize,
    pub scan_duration: Duration,
}

impl ScanStatistics {
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            total_bytes_transferred: 0,
            average_response_time: Duration::ZERO,
            fastest_response: Duration::MAX,
            slowest_response: Duration::ZERO,
            requests_per_second: 0.0,
            endpoints_discovered: 0,
            vulnerabilities_found: 0,
            vulnerabilities_by_type: HashMap::new(),
            vulnerabilities_by_severity: HashMap::new(),
            false_positives_filtered: 0,
            scan_duration: Duration::ZERO,
        }
    }

    pub fn record_request(&mut self, success: bool, response_time: Duration, bytes: u64) {
        self.total_requests += 1;

        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }

        self.total_bytes_transferred += bytes;

        if response_time < self.fastest_response {
            self.fastest_response = response_time;
        }

        if response_time > self.slowest_response {
            self.slowest_response = response_time;
        }

        // Update average (simplified - should use running average)
        let total_time = self.average_response_time * self.total_requests as u32
            + response_time;
        self.average_response_time = total_time / (self.total_requests + 1) as u32;
    }

    pub fn calculate_throughput(&mut self, scan_duration: Duration) {
        self.scan_duration = scan_duration;
        if scan_duration.as_secs() > 0 {
            self.requests_per_second = self.total_requests as f64 / scan_duration.as_secs_f64();
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        (self.successful_requests as f64 / self.total_requests as f64) * 100.0
    }

    pub fn average_bytes_per_request(&self) -> u64 {
        if self.total_requests == 0 {
            return 0;
        }
        self.total_bytes_transferred / self.total_requests
    }

    pub fn efficiency_score(&self) -> f64 {
        // Score based on vuln found per request
        if self.total_requests == 0 {
            return 0.0;
        }
        (self.vulnerabilities_found as f64 / self.total_requests as f64) * 100.0
    }

    pub fn summary(&self) -> String {
        format!(
            "Scan Statistics:\n\
            Total Requests: {}\n\
            Success Rate: {:.2}%\n\
            Avg Response Time: {:?}\n\
            Throughput: {:.2} req/s\n\
            Vulnerabilities Found: {}\n\
            Efficiency: {:.4}% (vulns/request)",
            self.total_requests,
            self.success_rate(),
            self.average_response_time,
            self.requests_per_second,
            self.vulnerabilities_found,
            self.efficiency_score()
        )
    }
}

impl Default for ScanStatistics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics() {
        let mut stats = ScanStatistics::new();

        stats.record_request(true, Duration::from_millis(100), 1024);
        stats.record_request(true, Duration::from_millis(200), 2048);
        stats.record_request(false, Duration::from_millis(50), 512);

        assert_eq!(stats.total_requests, 3);
        assert_eq!(stats.successful_requests, 2);
        assert_eq!(stats.failed_requests, 1);
        assert_eq!(stats.success_rate(), 66.66666666666666);
    }
}
