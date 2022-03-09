## ya-vm-file-server

Cross-platform 9p file server implemented in Rust/Tokio.

The work is based on:

https://github.com/pfpacket/rust-9p

Author of original implementation: Ryo Munakata

## why separate project

We have to fork this library to enable cross-platform capabilities of p9 server. 
Original implementation worked only of Linux filesystem.
* Emulating unix attributes
* Maximum separation between server filesystem and client filesystem
* Similar behaviour when served on Linux and Windows machine.

## Build

To compile only library part:

```javascript
cargo build
```

To compile binary:

```javascript
cargo build --bin ya-vm-file-server --features="build-binary"
```

## Testing

todo

## Licence

Keeping original implementation licence BSD-3

