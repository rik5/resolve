use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct HostEntry {
    pub ip: String,
    pub names: Vec<String>,
    #[allow(dead_code)]
    pub source: String, // "/etc/hosts"
}

pub fn parse_hosts_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<HostEntry>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue; // Invalid line
        }

        let ip = parts[0].to_string();
        let names = parts[1..].iter().map(|s| s.to_string()).collect();

        entries.push(HostEntry {
            ip,
            names,
            source: "/etc/hosts".to_string(),
        });
    }

    Ok(entries)
}

pub fn resolve_host_from_hosts<'a>(name: &str, entries: &'a [HostEntry]) -> Vec<&'a HostEntry> {
    entries
        .iter()
        .filter(|entry| entry.names.contains(&name.to_string()))
        .collect()
}