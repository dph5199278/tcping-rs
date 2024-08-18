[![Release](https://img.shields.io/github/v/release/dph5199278/tcping-rs)](https://github.com/dph5199278/tcping-rs/releases/latest)
[![Release](https://img.shields.io/github/license/dph5199278/tcping-rs)](https://github.com/dph5199278/tcping-rs/blob/main/LICENSE)

# Tcping-rs

Simple utility to ping a TCP port.

## Example
```sh
> tcping -c 4 google.com
Probing google.com(172.217.15.238:80) - Port is open - time=1.089ms
Probing google.com(172.217.15.238:80) - Port is open - time=1.533ms
Probing google.com(172.217.15.238:80) - Port is open - time=1.138ms
Probing google.com(172.217.15.238:80) - Port is open - time=1.344ms

Ping statistics for 172.217.15.238:80
     4 probes sent.
     4 successful, 0 failed.  (0.00% fail)
Approximate trip times in milli-seconds:
     Minimum = 1.533ms, Maximum = 1.089ms, Average = 1.276ms
```

## Usage
```sh
> tcping --help
Usage: tcping <host> [<port>] [-4] [-6] [-i <interval>] [-t <timeout>] [-c <cou
t>] [-d]

TCP ping utility.

Positional Arguments:
  host              target host
  port              target port (Default 80)

Options:
  -4, --only-ipv4 only ipv4
  -6, --only-ipv6 only ipv6
  -i, --interval    ping interval (Default 1)
  -t, --timeout     handshake timeout (Default 4)
  -c, --count       stop after sending N pings
  -d, --datetime    include date and time on each line
  --help            display usage information
```
