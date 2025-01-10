use std::net::IpAddr;

use dns_lookup::lookup_host;

pub fn lookup_ip(host: String, only_ipv4: bool, only_ipv6: bool) -> Result<IpAddr, String> {
    // Try parsing as IP address first
    if let Ok(ip) = host.parse::<IpAddr>() {
        return Ok(ip);
    }

    // Lookup IP by domain
    let ips = lookup_host(host.as_str())
        .map_err(|_| format!("DNS: Could not find host - {host:?}, aborting"))?;

    if ips.is_empty() {
        return Err(format!("DNS: Could not find host - {host:?}, aborting"));
    }

    // If no specific IP version is required, or both are required, return first result
    if (!only_ipv4 && !only_ipv6) || (only_ipv4 && only_ipv6) {
        return Ok(ips[0]);
    }

    // Find first matching IP version
    ips.into_iter()
        .find(|ip| (only_ipv4 && ip.is_ipv4()) || (only_ipv6 && ip.is_ipv6()))
        .ok_or_else(|| "DNS: No valid host found in AddrInfo for that type".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_ip() {
      assert_eq!(lookup_ip("google.com".to_string(), false, false).is_ok(), true);
    }
}
