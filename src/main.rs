mod cli;
mod nss;
mod dns;
mod proc;
mod explain;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ResolutionResult {
    name: String,
    result: Option<String>,
    steps: Vec<explain::decision_tree::DecisionStep>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Command::Host { name, pid, why } => {
            let result = resolve_host(&name, pid, why).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                if let Some(ip) = &result.result {
                    println!("{} → {}", name, ip);
                } else {
                    println!("{} not resolved", name);
                }
                if why {
                    println!("Resolution path:");
                    for (i, step) in result.steps.iter().enumerate() {
                        let outcome_str = match &step.outcome {
                            explain::decision_tree::Outcome::Match(ip) => format!("Match: {}", ip),
                            explain::decision_tree::Outcome::NoMatch => "No match".to_string(),
                            explain::decision_tree::Outcome::Error(e) => format!("Error: {}", e),
                            explain::decision_tree::Outcome::Unsupported(r) => format!("Unsupported: {}", r),
                        };
                        println!("  {}. {} → {}", i + 1, step.source, outcome_str);
                        if !step.reason.is_empty() {
                            println!("     Reason: {}", step.reason);
                        }
                    }
                }
            }
        }
        cli::Command::User { name, pid, why } => {
            let result = resolve_user(&name, pid).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                if let Some(uid) = &result.result {
                    println!("{} → uid {}", name, uid);
                } else {
                    println!("{} not found", name);
                }
                if why {
                    println!("Resolution path:");
                    for (i, step) in result.steps.iter().enumerate() {
                        let outcome_str = match &step.outcome {
                            explain::decision_tree::Outcome::Match(uid) => format!("Match: {}", uid),
                            explain::decision_tree::Outcome::NoMatch => "No match".to_string(),
                            explain::decision_tree::Outcome::Error(e) => format!("Error: {}", e),
                            explain::decision_tree::Outcome::Unsupported(r) => format!("Unsupported: {}", r),
                        };
                        println!("  {}. {} → {}", i + 1, step.source, outcome_str);
                        if !step.reason.is_empty() {
                            println!("     Reason: {}", step.reason);
                        }
                    }
                }
            }
        }
        cli::Command::Group { name, pid, why } => {
            let result = resolve_group(&name, pid).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                if let Some(gid) = &result.result {
                    println!("{} → gid {}", name, gid);
                } else {
                    println!("{} not found", name);
                }
                if why {
                    println!("Resolution path:");
                    for (i, step) in result.steps.iter().enumerate() {
                        let outcome_str = match &step.outcome {
                            explain::decision_tree::Outcome::Match(gid) => format!("Match: {}", gid),
                            explain::decision_tree::Outcome::NoMatch => "No match".to_string(),
                            explain::decision_tree::Outcome::Error(e) => format!("Error: {}", e),
                            explain::decision_tree::Outcome::Unsupported(r) => format!("Unsupported: {}", r),
                        };
                        println!("  {}. {} → {}", i + 1, step.source, outcome_str);
                        if !step.reason.is_empty() {
                            println!("     Reason: {}", step.reason);
                        }
                    }
                }
            }
        }
        cli::Command::Diff { pid: _pid, pid2: _pid2 } => {
            // TODO: Implement diff
            println!("Diff not implemented yet");
        }
    }

    Ok(())
}

async fn resolve_user(name: &str, _pid: Option<u32>) -> anyhow::Result<ResolutionResult> {
    let mut steps = Vec::new();

    // For simplicity, parse /etc/passwd
    let users = nss::passwd::parse_passwd_file("/etc/passwd").unwrap_or_default();
    let matches = nss::passwd::resolve_user_from_passwd(name, &users);
    if let Some(entry) = matches.first() {
        steps.push(explain::decision_tree::DecisionStep {
            source: "files (/etc/passwd)".to_string(),
            outcome: explain::decision_tree::Outcome::Match(entry.uid.to_string()),
            reason: "Found in passwd file".to_string(),
        });
        return Ok(ResolutionResult {
            name: name.to_string(),
            result: Some(entry.uid.to_string()),
            steps,
        });
    } else {
        steps.push(explain::decision_tree::DecisionStep {
            source: "files (/etc/passwd)".to_string(),
            outcome: explain::decision_tree::Outcome::NoMatch,
            reason: "Not found in passwd file".to_string(),
        });
    }

    Ok(ResolutionResult {
        name: name.to_string(),
        result: None,
        steps,
    })
}

async fn resolve_group(name: &str, _pid: Option<u32>) -> anyhow::Result<ResolutionResult> {
    let mut steps = Vec::new();

    // For simplicity, parse /etc/group
    let groups = nss::group::parse_group_file("/etc/group").unwrap_or_default();
    let matches = nss::group::resolve_group_from_group(name, &groups);
    if let Some(entry) = matches.first() {
        steps.push(explain::decision_tree::DecisionStep {
            source: "files (/etc/group)".to_string(),
            outcome: explain::decision_tree::Outcome::Match(entry.gid.to_string()),
            reason: "Found in group file".to_string(),
        });
        return Ok(ResolutionResult {
            name: name.to_string(),
            result: Some(entry.gid.to_string()),
            steps,
        });
    } else {
        steps.push(explain::decision_tree::DecisionStep {
            source: "files (/etc/group)".to_string(),
            outcome: explain::decision_tree::Outcome::NoMatch,
            reason: "Not found in group file".to_string(),
        });
    }

    Ok(ResolutionResult {
        name: name.to_string(),
        result: None,
        steps,
    })
}

async fn resolve_host(name: &str, _pid: Option<u32>, _why: bool) -> anyhow::Result<ResolutionResult> {
    let mut steps = Vec::new();

    // Parse nsswitch
    let mut nss_order = nss::nsswitch::parse_nsswitch_file("/etc/nsswitch.conf").unwrap_or_default();
    if nss_order.hosts.is_empty() {
        nss_order.hosts = vec!["files".to_string(), "dns".to_string()];
    }

    for source in &nss_order.hosts {
        match source.as_str() {
            "files" => {
                let hosts = nss::hosts::parse_hosts_file("/etc/hosts").unwrap_or_default();
                let matches = nss::hosts::resolve_host_from_hosts(name, &hosts);
                if let Some(entry) = matches.first() {
                    steps.push(explain::decision_tree::DecisionStep {
                        source: "files (/etc/hosts)".to_string(),
                        outcome: explain::decision_tree::Outcome::Match(entry.ip.clone()),
                        reason: "Found in hosts file".to_string(),
                    });
                    return Ok(ResolutionResult {
                        name: name.to_string(),
                        result: Some(entry.ip.clone()),
                        steps,
                    });
                } else {
                    steps.push(explain::decision_tree::DecisionStep {
                        source: "files (/etc/hosts)".to_string(),
                        outcome: explain::decision_tree::Outcome::NoMatch,
                        reason: "Not found in hosts file".to_string(),
                    });
                }
            }
            "dns" => {
                // Try systemd-resolved if Linux
                steps.push(explain::decision_tree::DecisionStep {
                    source: "dns (systemd-resolved)".to_string(),
                    outcome: if cfg!(target_os = "linux") {
                        match dns::resolved::resolve_hostname_via_resolved(name).await {
                            Ok(ip) => explain::decision_tree::Outcome::Match(ip),
                            Err(e) => explain::decision_tree::Outcome::Error(format!("DBus error: {}", e)),
                        }
                    } else {
                        explain::decision_tree::Outcome::Unsupported("systemd-resolved is Linux-only".to_string())
                    },
                    reason: if cfg!(target_os = "linux") { "Attempted systemd-resolved" } else { "Skipped on non-Linux" }.to_string(),
                });

                // If not matched, try libc DNS
                if !matches!(steps.last().unwrap().outcome, explain::decision_tree::Outcome::Match(_)) {
                    match dns::resolved::resolve_hostname_libc(name).await {
                        Ok(ip) => {
                            steps.push(explain::decision_tree::DecisionStep {
                                source: "dns (libc)".to_string(),
                                outcome: explain::decision_tree::Outcome::Match(ip.clone()),
                                reason: "Resolved using system resolver".to_string(),
                            });
                            return Ok(ResolutionResult {
                                name: name.to_string(),
                                result: Some(ip),
                                steps,
                            });
                        }
                        Err(e) => {
                            steps.push(explain::decision_tree::DecisionStep {
                                source: "dns (libc)".to_string(),
                                outcome: explain::decision_tree::Outcome::Error(format!("Libc error: {}", e)),
                                reason: "Failed to resolve via libc".to_string(),
                            });
                        }
                    }
                } else {
                    // If systemd matched, return it
                    if let explain::decision_tree::Outcome::Match(ip) = &steps.last().unwrap().outcome {
                        return Ok(ResolutionResult {
                            name: name.to_string(),
                            result: Some(ip.clone()),
                            steps,
                        });
                    }
                }
            }
            _ => {
                steps.push(explain::decision_tree::DecisionStep {
                    source: format!("{} (unsupported)", source),
                    outcome: explain::decision_tree::Outcome::NoMatch,
                    reason: "Source not implemented".to_string(),
                });
            }
        }
    }

    Ok(ResolutionResult {
        name: name.to_string(),
        result: None,
        steps,
    })
}