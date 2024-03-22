Some shitty container implementation from scratch.


## Usage
1. Get a filesystem of your choice and put it in a directory named `rootfs`.
```bash
mkdir rootfs
tar -xzvf ubuntu-base-22.04-base-amd64.tar.gz -C rootfs
```
2. Run the following command to build and run the container:
```bash
cargo build && sudo ./target/debug/containers run /bin/bash
```
