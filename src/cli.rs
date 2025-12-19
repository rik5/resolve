use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    #[arg(global = true, long)]
    pub json: bool,
}

#[derive(Subcommand)]
pub enum Command {
    Host {
        name: String,
        #[arg(long)]
        pid: Option<u32>,
        #[arg(long)]
        why: bool,
    },
    User {
        name: String,
        #[arg(long)]
        pid: Option<u32>,
        #[arg(long)]
        why: bool,
    },
    Group {
        name: String,
        #[arg(long)]
        pid: Option<u32>,
        #[arg(long)]
        why: bool,
    },
    Diff {
        #[arg(long)]
        pid: u32,
        #[arg(long)]
        pid2: u32,
    },
}