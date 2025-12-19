use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct UserEntry {
    pub name: String,
    pub uid: u32,
    #[allow(dead_code)]
    pub gid: u32,
    #[allow(dead_code)]
    pub gecos: String,
    #[allow(dead_code)]
    pub home: String,
    #[allow(dead_code)]
    pub shell: String,
    #[allow(dead_code)]
    pub source: String,
}

pub fn parse_passwd_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<UserEntry>> {
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
        if parts.len() < 7 {
            continue;
        }

        let name = parts[0].to_string();
        let uid = parts[2].parse().unwrap_or(0);
        let gid = parts[3].parse().unwrap_or(0);
        let gecos = parts[4].to_string();
        let home = parts[5].to_string();
        let shell = parts[6].to_string();

        entries.push(UserEntry {
            name,
            uid,
            gid,
            gecos,
            home,
            shell,
            source: "/etc/passwd".to_string(),
        });
    }

    Ok(entries)
}

pub fn resolve_user_from_passwd<'a>(name: &str, entries: &'a [UserEntry]) -> Vec<&'a UserEntry> {
    entries
        .iter()
        .filter(|entry| entry.name == name)
        .collect()
}