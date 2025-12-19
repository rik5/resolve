use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct GroupEntry {
    pub name: String,
    pub gid: u32,
    #[allow(dead_code)]
    pub members: Vec<String>,
    #[allow(dead_code)]
    pub source: String,
}

pub fn parse_group_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<GroupEntry>> {
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

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 4 {
            continue;
        }

        let name = parts[0].to_string();
        let gid = parts[2].parse().unwrap_or(0);
        let members: Vec<String> = if parts[3].is_empty() {
            Vec::new()
        } else {
            parts[3].split(',').map(|s| s.to_string()).collect()
        };

        entries.push(GroupEntry {
            name,
            gid,
            members,
            source: "/etc/group".to_string(),
        });
    }

    Ok(entries)
}

pub fn resolve_group_from_group<'a>(name: &str, entries: &'a [GroupEntry]) -> Vec<&'a GroupEntry> {
    entries
        .iter()
        .filter(|entry| entry.name == name)
        .collect()
}