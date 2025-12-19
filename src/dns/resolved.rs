use zbus::{dbus_proxy, Connection};
use anyhow::Result;

#[dbus_proxy(
    interface = "org.freedesktop.resolve1",
    default_service = "org.freedesktop.resolve1",
    default_path = "/org/freedesktop/resolve1"
)]
trait Resolve1 {
    async fn resolve_hostname(&self, ifindex: i32, name: &str, family: i32, flags: u64) -> zbus::Result<(Vec<String>, String, u32)>;
    // Other methods as needed
}

pub async fn resolve_hostname_via_resolved(name: &str) -> Result<String> {
    let connection = Connection::system().await?;
    let proxy = Resolve1Proxy::new(&connection).await?;
    let (addresses, _, _) = proxy.resolve_hostname(0, name, 0, 0).await?;
    Ok(addresses.into_iter().next().unwrap_or_default())
}

pub async fn resolve_hostname_libc(name: &str) -> anyhow::Result<String> {
    use tokio::net::lookup_host;
    let addrs = lookup_host((name, 0)).await?;
    for addr in addrs {
        if let std::net::IpAddr::V4(ip) = addr.ip() {
            return Ok(ip.to_string());
        }
    }
    Err(anyhow::anyhow!("No IPv4 address found"))
}