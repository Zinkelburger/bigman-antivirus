use regex::Regex;
use anyhow::Result;

pub struct AnalysisResult {
    pub is_safe: bool,
    pub threats: Vec<String>,
}

pub fn analyze_content(content: &str) -> Result<AnalysisResult> {
    let mut threats = Vec::new();

    // Check for dangerous patterns
    threats.extend(check_dangerous_patterns(content)?);

    let is_safe = threats.is_empty();

    Ok(AnalysisResult {
        is_safe,
        threats,
    })
}


fn check_dangerous_patterns(content: &str) -> Result<Vec<String>> {
    let mut threats = Vec::new();

    // Look for any potentially risky commands and explain them clearly
    let patterns = vec![
        (r"curl\s+[^\s]+", "Downloads files from the internet"),
        (r"wget\s+[^\s]+", "Downloads files from the internet"),
        (r"curl\s+.*\|\s*(bash|sh|zsh)", "Downloads and immediately executes code (VERY DANGEROUS)"),
        (r"wget\s+.*\|\s*(bash|sh|zsh)", "Downloads and immediately executes code (VERY DANGEROUS)"),
        (r"rm\s+-r[f]?", "Recursively deletes files/folders (can be destructive)"),
        (r"rm\s+.*\*", "Deletes files using wildcards (can delete more than intended)"),
        (r"sudo\s+", "Runs commands with administrator privileges"),
        (r"chmod\s+", "Changes file permissions"),
        (r"chmod\s+(777|666)", "Makes files readable/writable by everyone (security risk)"),
        (r"base64\s+-d", "Decodes hidden/obfuscated content"),
        (r"echo\s+.*\|\s*base64", "Decodes hidden/obfuscated content"),
        (r"nc\s+-l|netcat\s+-l", "Opens network port (potential backdoor)"),
        (r"crontab\s+", "Modifies scheduled tasks"),
        (r"\.ssh/", "Accesses SSH keys (remote access credentials)"),
        (r"/etc/passwd", "Accesses user account information"),
        (r"systemctl\s+enable", "Enables system services"),
    ];

    for (pattern, description) in &patterns {
        if let Ok(regex) = Regex::new(pattern) {
            for mat in regex.find_iter(content) {
                threats.push(format!("{}: {}", mat.as_str(), description));
            }
        }
    }

    Ok(threats)
}

 