use std::net::IpAddr;

use dns_lookup::lookup_host;

pub fn lookup_ip(host: String, only_ipv4: bool, only_ipv6: bool) -> Result<IpAddr, String> {
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
    if (!only_ipv4 && !only_ipv6) || (only_ipv4 && only_ipv6) {
        return Ok(system_resolver_response[0]);
    }
    for ans in system_resolver_response.iter() {
        if only_ipv4 && ans.is_ipv4() {
            return Ok(*ans);
        }
        if only_ipv6 && ans.is_ipv6() {
            return Ok(*ans);
        }
    }
    return Err("DNS: No valid host found in AddrInfo for that type".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_ip() {
      assert_eq!(lookup_ip("google.com".to_string(), false, false).is_ok(), true);
    }
}
