use argh::FromArgs;

use chrono::Local;  

use std::net::{SocketAddr, TcpStream, Shutdown};
use std::time::Duration;

mod utils {
    pub mod net;
}
use utils::net::lookup_ip;

#[derive(FromArgs)]
/// TCP ping utility.
struct TcpPing {
    /// target host
    #[argh(positional)]
    host: String,
    /// target port (Default 80)
    #[argh(positional, default = "80")]
    port: u16,
    /// only ipv4
    #[argh(switch, short = '4')]
    prefer_ipv4: bool,
    /// only ipv6
    #[argh(switch, short = '6')]
    prefer_ipv6: bool,
    /// ping interval (Default 1)
    #[argh(option, short = 'i', default = "1")]
    interval: u64,
    /// handshake timeout (Default 4)
    #[argh(option, short = 't', default = "4")]
    timeout: u64,
    /// stop after sending N pings
    #[argh(option, short = 'c')]
    count: Option<usize>,
    /// include date and time on each line
    #[argh(switch, short = 'd')]
    datetime: bool,
}

fn main() {
    let args: TcpPing = argh::from_env();

    // get ip by domain
    let ip_result = lookup_ip(args.host.clone(), args.prefer_ipv4, args.prefer_ipv6);
    if let Err(msg) = ip_result {
        // get ip err
        println!("{}", msg);
        std::process::exit(1);
    }
    let addr = SocketAddr::new(ip_result.unwrap(), args.port);

    let timeout: Duration = Duration::from_secs(args.timeout);
    let mut total_pings = 0;
    let mut success_pings = 0;
    let mut fail_pings = 0;
    let mut list = Vec::new();
    loop {
        let mut datetime = String::new();
        if args.datetime {
            datetime = format!("{}: ", Local::now().format("%Y-%m-%d %H:%M:%S"));
        }
        let start = std::time::Instant::now();
        let res = TcpStream::connect_timeout(&addr, timeout);
        let elapsed = std::time::Instant::now().duration_since(start);
        let mut resp_msg = "No response";
        if let Ok(stream) = res {
            let _ = stream.shutdown(Shutdown::Both);
            resp_msg = "Port is open";
            success_pings += 1;
        }
        else {
            fail_pings += 1;
        }
        let cost = elapsed.as_nanos() as f64 / 1_000_000.0;
        list.push(cost);
        println!("{}Probing {}({}) - {} - time={:.3}ms", &datetime, args.host.clone(), &addr, resp_msg, cost);
        total_pings += 1;
        if let Some(c) = args.count {
            if total_pings >= c {
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(args.interval));
    }

    let count = args.count.unwrap_or(0);
    let min = list.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let max = list.iter().cloned().fold(f64::INFINITY, f64::min);
    let avg = list.iter().sum::<f64>() / list.len() as f64;
    println!("");
    println!("Ping statistics for {}", &addr);
    println!("     {} probes sent.", count);
    println!("     {} successful, {} failed.  ({:.2}% fail)", success_pings, fail_pings, fail_pings as f32 / count as f32);
    println!("Approximate trip times in milli-seconds:");
    println!("     Minimum = {:.3}ms, Maximum = {:.3}ms, Average = {:.3}ms", min, max, avg);

    std::process::exit(0);
}
