#[cfg(test)]
mod tests {
    use crate::model::VulnAnalysisModel;

    #[test]
    fn test_model_creation() {
        let model = VulnAnalysisModel::new(true);
        assert!(true); // Model creation should not panic
    }

    #[test]
    fn test_disabled_model() {
        let model = VulnAnalysisModel::new(false);
        let (confidence, analysis) = model.analyze_vulnerability("SQL Injection", "test", "test");
        assert_eq!(confidence, 0.5);
        assert!(analysis.contains("disabled"));
    }

    #[test]
    fn test_analyze_sqli_with_strong_evidence() {
        let model = VulnAnalysisModel::new(true);
        let evidence = "mysql syntax error";
        let (confidence, analysis) = model.analyze_vulnerability(
            "SQL Injection",
            evidence,
            "mysql error: syntax error near 'SELECT'",
        );

        assert!(confidence > 0.5);
        assert!(analysis.contains("SQL Injection"));
    }

    #[test]
    fn test_analyze_xss() {
        let model = VulnAnalysisModel::new(true);
        let evidence = "<script>alert(1)</script>";
        let (confidence, _) = model.analyze_vulnerability("XSS", evidence, evidence);

        assert!(confidence > 0.7);
    }

    #[test]
    fn test_generate_poc() {
        let model = VulnAnalysisModel::new(true);
        let poc = model.generate_poc("SQL Injection", "http://test.com", "' OR 1=1--");

        assert!(poc.is_some());
        let poc_str = poc.unwrap();
        assert!(poc_str.contains("curl"));
        assert!(poc_str.contains("test.com"));
    }

    #[test]
    fn test_generate_poc_disabled() {
        let model = VulnAnalysisModel::new(false);
        let poc = model.generate_poc("SQL Injection", "http://test.com", "' OR 1=1--");

        assert!(poc.is_none());
    }

    #[test]
    fn test_confidence_ranges() {
        let model = VulnAnalysisModel::new(true);

        // High confidence case
        let (high_conf, _) = model.analyze_vulnerability(
            "LFI",
            "root:x:0:0:root:/root:/bin/bash",
            "root:x:0:0:root:/root:/bin/bash",
        );
        assert!(high_conf >= 0.8);

        // Low confidence case
        let (low_conf, _) = model.analyze_vulnerability(
            "SQL Injection",
            "",
            "normal response",
        );
        assert!(low_conf < 0.8);
    }
}
