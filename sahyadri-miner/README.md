# sydar-miner
[![Build status](https://github.com/elichai/sydar-miner/workflows/ci/badge.svg)](https://github.com/elichai/sydar-miner/actions)
[![Latest version](https://img.shields.io/crates/v/sydar-miner.svg)](https://crates.io/crates/sydar-miner)
![License](https://img.shields.io/crates/l/sydar-miner.svg)
[![dependency status](https://deps.rs/repo/github/elichai/sydar-miner/status.svg)](https://deps.rs/repo/github/elichai/sydar-miner)

A Rust binary for file encryption to multiple participants. 


## Installation
### From Sources
With Rust's package manager cargo, you can install sydar-miner via:

```sh
cargo install sydar-miner
```

### From Binaries
The [release page](https://github.com/elichai/sydar-miner/releases) includes precompiled binaries for Linux, macOS and Windows.


# Usage
To start mining you need to run [sydard](https://github.com/sydarnet/sydard) and have an address to send the rewards to.
There's a guide here on how to run a full node and how to generate addresses: https://github.com/sydarnet/docs/blob/main/Getting%20Started/Full%20Node%20Installation.md

Help:
```
sydar-miner 0.2.1
A sydar high performance CPU miner

USAGE:
    sydar-miner [FLAGS] [OPTIONS] --mining-address <mining-address>

FLAGS:
    -d, --debug                   Enable debug logging level
    -h, --help                    Prints help information
        --mine-when-not-synced    Mine even when sydard says it is not synced, only useful when passing `--allow-submit-
                                  block-when-not-synced` to sydard  [default: false]
        --testnet                 Use testnet instead of mainnet [default: false]
    -V, --version                 Prints version information

OPTIONS:
        --devfund <devfund-address>            Mine a percentage of the blocks to the sydar devfund [default: Off]
        --devfund-percent <devfund-percent>    The percentage of blocks to send to the devfund [default: 1]
    -s, --sydard-address <sydard-address>      The IP of the sydard instance [default: 127.0.0.1]
    -a, --mining-address <mining-address>      The sydar address for the miner reward
    -t, --threads <num-threads>                Amount of miner threads to launch [default: number of logical cpus]
    -p, --port <port>                          sydard port [default: Mainnet = 16111, Testnet = 16211]
```

To start mining you just need to run the following:

`./sydar-miner --mining-address sydar:XXXXX`

This will run the miner on all the available CPU cores.

# Devfund
**NOTE: This feature is off by default** <br>
The devfund is a fund managed by the sydar community in order to fund sydar development <br>
A miner that wants to mine a percentage into the dev-fund can pass the following flags: <br>
`sydar-miner --mining-address= XXX --devfund=sydar:precqv0krj3r6uyyfa36ga7s0u9jct0v4wg8ctsfde2gkrsgwgw8jgxfzfc98` <br>
and can pass `--devfund-precent=XX.YY` to mine only XX.YY% of the blocks into the devfund (passing `--devfund` without specifying a percent will default to 1%)

# Donation Address
`sydar:qzvqtx5gkvl3tc54up6r8pk5mhuft9rtr0lvn624w9mtv4eqm9rvc9zfdmmpu`