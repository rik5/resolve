use std::fs;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NamespaceInfo {
    pub mnt_ns: String,
    pub net_ns: String,
}

#[allow(dead_code)]
pub fn get_namespace_info(pid: u32) -> std::io::Result<NamespaceInfo> {
    let mnt_path = format!("/proc/{}/ns/mnt", pid);
    let net_path = format!("/proc/{}/ns/net", pid);

    let mnt_ns = fs::read_link(&mnt_path)?;
    let net_ns = fs::read_link(&net_path)?;

    Ok(NamespaceInfo {
        mnt_ns: mnt_ns.to_string_lossy().to_string(),
        net_ns: net_ns.to_string_lossy().to_string(),
    })
}

#[allow(dead_code)]
pub fn namespaces_differ(pid: u32) -> std::io::Result<bool> {
    let proc_ns = get_namespace_info(pid)?;
    let self_ns = get_namespace_info(std::process::id())?;

    Ok(proc_ns.mnt_ns != self_ns.mnt_ns || proc_ns.net_ns != self_ns.net_ns)
}

#[allow(dead_code)]
pub fn get_proc_root_path(pid: u32, relative_path: &str) -> String {
    format!("/proc/{}/root{}", pid, relative_path)
}