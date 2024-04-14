async function get_sysinfo() {
    let response = await fetch("/api/sysinfo");
    return await response.json()
}
