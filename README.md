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

```bash
cargo build
```

To compile binary:

```bash
cargo build --bin ya-vm-file-server --features="build-binary"
```

## Testing

Build docker:

```
docker build . -t ya-vm-file-server
```

Running docker tests

```
docker-compose up
```

### Integration tests
Tests are half-automatic. First you need to prepare an environment:
1) Launch 9p server:
   
   ```
   RUST_LOG=debug cargo run -- --mount-point tests/9p_mnt_point
   ```
   
3) On other shell mount to the server:
    
   ``` 
   cd tests
   sudo mount -t 9p -o version=9p2000.L,trans=tcp,debug=0x04,port=7878,uname=testuser 127.0.0.1 ./mnt_tests
   ```

4) Launch tests using **nightly** build:

   ```
   cargo +nightly test 
   ```

You should be able to see logging on the server side while tests are running.


## Licence

Keeping original implementation licence BSD-3

