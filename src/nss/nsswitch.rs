use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct NssOrder {
    pub hosts: Vec<String>,
    #[allow(dead_code)]
    pub passwd: Vec<String>,
    #[allow(dead_code)]
    pub group: Vec<String>,
}

pub fn parse_nsswitch_file<P: AsRef<Path>>(path: P) -> io::Result<NssOrder> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            let sources: Vec<String> = value
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            map.insert(key.to_string(), sources);
        }
    }

    Ok(NssOrder {
        hosts: map.get("hosts").cloned().unwrap_or_default(),
        passwd: map.get("passwd").cloned().unwrap_or_default(),
        group: map.get("group").cloned().unwrap_or_default(),
    })
}