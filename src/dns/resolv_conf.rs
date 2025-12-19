use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ResolvConf {
    pub nameservers: Vec<String>,
    pub search_domains: Vec<String>,
    pub domain: Option<String>,
    pub options: Vec<String>,
}

#[allow(dead_code)]
pub fn parse_resolv_conf<P: AsRef<Path>>(path: P) -> io::Result<ResolvConf> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut nameservers = Vec::new();
    let mut search_domains = Vec::new();
    let mut domain = None;
    let mut options = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "nameserver" if parts.len() > 1 => {
                nameservers.push(parts[1].to_string());
            }
            "search" => {
                search_domains.extend(parts[1..].iter().map(|s| s.to_string()));
            }
            "domain" if parts.len() > 1 => {
                domain = Some(parts[1].to_string());
            }
            "options" => {
                options.extend(parts[1..].iter().map(|s| s.to_string()));
            }
            _ => {}
        }
    }

    Ok(ResolvConf {
        nameservers,
        search_domains,
        domain,
        options,
    })
}