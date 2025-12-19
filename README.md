# resolve

A deterministic identity-resolution inspector for Linux and cross-platform systems.

`resolve` tells you exactly how and why systems resolve hostnames, users, groups, and services — across DNS, NSS, namespaces, and caches. No shelling out, read-only operations, fully deterministic.

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (install via rustup if not already installed)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/resolve.git
cd resolve

# Build the release binary
cargo build --release

# The binary will be available at target/release/resolve
./target/release/resolve --help
```

### Install Globally (Optional)

```bash
# After building, you can copy to a directory in your PATH
sudo cp target/release/resolve /usr/local/bin/
```

## Usage

### Host Resolution

```bash
# Basic hostname resolution
resolve host example.com

# With explanation
resolve host example.com --why

# JSON output
resolve host example.com --json
```

Example output:

```
example.com → 93.184.216.34
Resolution path:
  1. files (/etc/hosts) → No match
     Reason: Not found in hosts file
  2. dns (systemd-resolved) → Unsupported: systemd-resolved is Linux-only
     Reason: Skipped on non-Linux
  3. dns (libc) → Match: 93.184.216.34
     Reason: Resolved using system resolver
```

### User Resolution

```bash
resolve user root --why
```

Output:

```
root → uid 0
Resolution path:
  1. files (/etc/passwd) → Match: 0
     Reason: Found in passwd file
```

### Group Resolution

```bash
resolve group wheel --why
```

Output:

```
wheel → gid 0
Resolution path:
  1. files (/etc/group) → Match: 0
     Reason: Found in group file
```

### JSON Output

All commands support `--json` for structured output:

```json
{
  "name": "example.com",
  "result": "93.184.216.34",
  "steps": [
    {
      "source": "files (/etc/hosts)",
      "outcome": "NoMatch",
      "reason": "Not found in hosts file"
    },
    {
      "source": "dns (libc)",
      "outcome": {
        "Match": "93.184.216.34"
      },
      "reason": "Resolved using system resolver"
    }
  ]
}
```

## Features

- **Cross-platform**: Works on Linux (with systemd-resolved) and other Unix-like systems (with libc fallback)
- **Deterministic**: No external processes, read-only filesystem access
- **Transparent**: Explains every step of resolution with `--why`
- **Structured**: JSON output for programmatic use
- **NSS-aware**: Parses `/etc/nsswitch.conf` for resolution order
- **PID-scoped**: Future support for per-process resolution contexts

## Architecture

```
resolve/
├── cli.rs              # Command-line interface
├── main.rs             # Main logic and output formatting
├── nss/
│   ├── hosts.rs        # /etc/hosts parsing
│   ├── passwd.rs       # /etc/passwd parsing
│   ├── group.rs        # /etc/group parsing
│   └── nsswitch.rs     # NSS configuration
├── dns/
│   ├── resolved.rs     # systemd-resolved DBus client
│   └── resolv_conf.rs  # /etc/resolv.conf parsing (future)
├── proc/
│   └── namespaces.rs   # Namespace detection (future)
└── explain/
    └── decision_tree.rs # Resolution step tracking
```

## Roadmap

### Completed
- Host, user, group resolution
- Files-based NSS modules (/etc/hosts, /etc/passwd, /etc/group)
- DNS resolution (systemd-resolved + libc fallback)
- Cross-platform support
- --why human-readable explanations
- --json structured output
- NSS order parsing

### In Progress
- --pid flag for per-process resolution
- Diff mode for comparing resolver contexts

### Future
- Full NSS module support (LDAP, NIS, etc.)
- Container and Kubernetes awareness
- eBPF-based runtime tracing
- Security audit mode