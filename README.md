Some shitty container implementation from scratch.

#### This is some rootless container shit.

> **Note:-**
> In order to write the `gid_map` of a process we need to write `deny` to the `setgroups`.
> Due to this denying setgroups shit, we can't setgroup in the child process which is required in order to perform privilege task.


## Usage
1. Get a filesystem of your choice and put it in a directory named `rootfs`.
```bash
mkdir rootfs
tar -xzvf ubuntu-base-22.04-base-amd64.tar.gz -C rootfs
```
2. Run the following command to build and run the container:
```bash
cargo run run /bin/bash
```
