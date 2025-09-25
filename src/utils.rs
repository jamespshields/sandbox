use std::env;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use regex::Regex;

pub fn generate_container_name() -> String {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let mut hasher = DefaultHasher::new();
    current_dir.hash(&mut hasher);
    let hash = hasher.finish();
    let name = format!("sandbox-{:x}", hash & 0xFFFFFFFF);

    // Validate container name contains only safe characters
    validate_container_name(&name).unwrap_or_else(|| {
        panic!("Generated container name contains invalid characters: {}", name)
    })
}

pub fn validate_container_name(name: &str) -> Option<String> {
    // Docker container names must match: [a-zA-Z0-9][a-zA-Z0-9_.-]*
    let valid_name_regex = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_.-]*$").unwrap();

    if valid_name_regex.is_match(name) && name.len() <= 64 {
        Some(name.to_string())
    } else {
        None
    }
}

pub fn validate_claude_args(args: &[&str]) -> Result<Vec<String>, String> {
    let mut validated_args = Vec::new();

    // Regex to check for potentially dangerous patterns
    let dangerous_patterns = [
        r"[;&|`$(){}[\]<>]",  // Shell metacharacters
        r"^\s*-",             // Suspicious flags starting with dash
    ];

    for arg in args {
        // Check for dangerous patterns
        for pattern in &dangerous_patterns {
            let regex = Regex::new(pattern).unwrap();
            if regex.is_match(arg) {
                return Err(format!("Argument contains potentially dangerous characters: {}", arg));
            }
        }

        // Check for maximum argument length
        if arg.len() > 1000 {
            return Err("Argument too long".to_string());
        }

        validated_args.push(arg.to_string());
    }

    Ok(validated_args)
}