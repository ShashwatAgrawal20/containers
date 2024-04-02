use nix::{
    mount::{mount, umount, MsFlags},
    sched::{unshare, CloneFlags},
    unistd::{chdir, chroot, sethostname},
};
use std::{
    fs::File,
    io::Write,
    process::{exit, Command},
};

pub fn run(args: &Vec<String>) {
    println!("process id of parent:- {}", std::process::id());

    let uid = nix::unistd::getuid();
    let gid = nix::unistd::getgid();

    if let Err(err) = unshare(
        CloneFlags::CLONE_NEWUTS
            | CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWUSER,
    ) {
        eprintln!("Failed to create namespaces: {}", err);
        exit(1);
    }
    println!(
        "uid after namespace creation {:?}",
        nix::unistd::getuid().as_raw()
    );

    if let Err(err) = sethostname("container") {
        eprintln!("Failed to set hostname: {}", err);
        exit(1);
    }

    let pid = std::process::id();

    if let Ok(mut setgroups) = File::create(format!("/proc/{}/{}", pid, "setgroups")) {
        if let Err(err) = setgroups.write_all(b"deny") {
            eprintln!("Failed to write setgroups: {}", err);
        }
        if let Err(err) = setgroups.flush() {
            eprintln!("Failed to flush setgroups: {}", err);
        }
    } else {
        eprintln!("Failed to create setgroups");
        exit(1);
    }

    if let Ok(mut uid_map) = File::create(format!("/proc/{}/{}", pid, "uid_map")) {
        if let Err(err) = uid_map.write_all(format!("0 {} 1", uid.as_raw()).as_bytes()) {
            eprintln!("Failed to write uid_map: {}", err);
        }
        if let Err(err) = uid_map.flush() {
            eprintln!("Failed to flush uid_map: {}", err);
        }
    } else {
        eprintln!("Failed to create uid_map");
        exit(1);
    }

    // TODO: fix this shitty erroneous code
    if let Ok(mut gid_map) = File::create(format!("/proc/{}/{}", pid, "gid_map")) {
        if let Err(err) = gid_map.write_all(format!("0 {} 1", gid.as_raw()).as_bytes()) {
            eprintln!("Failed to write gid_map: {}", err);
            exit(1);
        }
        if let Err(err) = gid_map.flush() {
            eprintln!("Failed to flush gid_map: {}", err);
        }
    } else {
        eprintln!("Failed to create gid_map");
        exit(1);
    }

    // The `/proc/self/exe` is a symbolic link to the current process's executable.
    let _output = Command::new("/proc/self/exe")
        .args(format!("child {} {}", &args[2], &args[3..].join(" ")).split_whitespace())
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on process");
}

pub fn child(args: &Vec<String>) {
    println!("running {:?}", &args[2..]);
    println!("process id in child:- {}", std::process::id());

    let path = std::env::current_dir().unwrap();
    let abs_path = format!("{}/{}", path.display(), "rootfs");

    if let Err(err) = chroot(abs_path.as_str()) {
        eprintln!("Failed to chroot: {}", err);
        exit(1);
    }

    if let Err(err) = chdir("/") {
        eprintln!("Failed to change root directory: {}", err);
        exit(1);
    }

    if let Err(err) = mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::empty(),
        None::<&str>,
    ) {
        eprintln!("Failed to mount proc filesystem: {}", err);
        exit(1);
    }

    let _output = Command::new(&args[2])
        .args(&args[3..])
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on process");

    if let Err(err) = umount("/proc") {
        eprintln!("Failed to unmount proc filesystem: {}", err);
        exit(1);
    }
}
