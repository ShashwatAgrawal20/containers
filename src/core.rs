use nix::{
    mount::{mount, umount, umount2, MntFlags, MsFlags},
    sched::{unshare, CloneFlags},
    unistd::{chdir, pivot_root, sethostname},
};
use std::{
    fs::{create_dir_all, remove_dir},
    path::PathBuf,
    process::{exit, Command},
};

pub fn run(args: &Vec<String>) {
    println!("process id of parent:- {}", std::process::id());

    if let Err(err) =
        unshare(CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS)
    {
        eprintln!("Failed to create namespaces: {}", err);
        exit(1);
    }

    if let Err(err) = sethostname("container") {
        eprintln!("Failed to set hostname: {}", err);
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

    // TODO: Implement cgroups

    let path = std::env::current_dir().unwrap();
    let abs_path = format!("{}/{}", path.display(), "rootfs");
    let abs_path_pathbuf = PathBuf::from(&abs_path);

    // ----------------- Shitty pivot_root implementation -----------------
    if let Err(err) = mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_REC | MsFlags::MS_PRIVATE,
        None::<&str>,
    ) {
        eprintln!("Failed to remount root as private: {}", err);
        exit(1);
    };

    if let Err(err) = mount(
        Some(&abs_path_pathbuf),
        &abs_path_pathbuf,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_PRIVATE,
        None::<&str>,
    ) {
        eprintln!("Failed to remount root as private: {}", err);
        exit(1);
    };

    if let Err(err) = create_dir_all(format!("{}/oldrootfs", abs_path)) {
        eprintln!("Failed to create oldrootfs directory: {}", err);
        exit(1);
    };

    if let Err(err) = pivot_root(
        &abs_path_pathbuf,
        &PathBuf::from(format!("{}/oldrootfs", abs_path)),
    ) {
        eprintln!("Failed to pivot_root: {}", err);
        exit(1);
    };

    if let Err(err) = umount2(
        &std::path::PathBuf::from("/oldrootfs"),
        MntFlags::MNT_DETACH,
    ) {
        eprintln!("Failed to detach oldrootfs: {}", err);
        exit(1);
    };

    if let Err(err) = remove_dir(&PathBuf::from(format!("/oldrootfs"))) {
        eprintln!("Failed to remove oldrootfs: {}", err);
        exit(1);
    };
    if let Err(err) = chdir(&PathBuf::from("/")) {
        eprintln!("Failed to change directory to /: {}", err);
        exit(1);
    };
    // ----------------- Shitty pivot_root implementation -----------------

    if let Err(err) = mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::empty(),
        None::<&str>,
    ) {
        eprintln!("Failed to mount proc filesystem: {}", err);
        exit(1);
    };

    let _output = Command::new(&args[2])
        .args(&args[3..])
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on process");

    if let Err(err) = umount("/proc") {
        eprintln!("Failed to unmount proc filesystem: {}", err);
        exit(1);
    };
}
