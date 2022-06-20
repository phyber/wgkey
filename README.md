# wgkey

A Rust implementation of the Wireguard key generation commands.

## Why?

Boredom and wanting to play with something new in Rust.

## Usage

Usage of `wgkey` matches the official `wg` tool.

```
wgkey genkey | tee privkey | wgkey pubkey > pubkey
```

## Minimum Supported Rust Version (MSRV)

v1.58.1

## License

Licensed under either of

  * Apache License, Version 2.0
    ([LICENSE-APACHE] or https://www.apache.org/licenses/LICENSE-2.0)
  * MIT license
    ([LICENSE-MIT] or https://opensource.org/licenses/MIT)

at your option.

<!-- links -->
[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
