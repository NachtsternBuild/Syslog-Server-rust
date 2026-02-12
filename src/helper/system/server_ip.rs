use local_ip_address::local_ip;
use std::net::IpAddr;
/*
// Usage: let ip = server_ip();
pub fn server_ip() -> IpAddr {
	local_ip().expect("[ERROR] Fehler beim holen der IP-Addresse.")
}
*/
pub fn server_ip() -> Result<IpAddr, Box<dyn std::error::Error>> {
	Ok(local_ip()?)
}
