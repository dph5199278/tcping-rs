use std::net::IpAddr;

use dns_lookup::lookup_host;

pub fn lookup_ip(host: String, prefer_ipv4: bool, prefer_ipv6: bool) -> Result<IpAddr, String> {
    if let Ok(ip) = host.parse::<IpAddr>() {
        return Ok(ip);
    }

    // not ip, lookup ip by domain
    let system_resolver_response_result = lookup_host(host.as_str());
    if let Err(_) = system_resolver_response_result {
        return Err(format!("DNS: Could not find host - {:#?}, aborting", host));
    }
    let system_resolver_response = system_resolver_response_result.unwrap();
    if system_resolver_response.is_empty() {
        return Err(format!("DNS: Could not find host - {:#?}, aborting", host));
    }
    if (!prefer_ipv4 && !prefer_ipv6) || (prefer_ipv4 && prefer_ipv6) {
        return Ok(system_resolver_response[0]);
    }
    for ans in system_resolver_response.iter() {
        if prefer_ipv4 && ans.is_ipv4() {
            return Ok(*ans);
        }
        if prefer_ipv6 && ans.is_ipv6() {
            return Ok(*ans);
        }
    }
    return Err("DNS: No valid host found in AddrInfo for that type".to_string());
}
