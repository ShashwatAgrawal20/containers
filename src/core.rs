use nix::sched::{unshare, CloneFlags};
use nix::unistd::sethostname;
use std::process::{exit, Command};

pub fn run(args: &Vec<String>) {
    println!("process id of parent:- {}", std::process::id());

    if let Err(err) = unshare(CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID) {
        eprintln!("Failed to create UTS namespace: {}", err);
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

    let _output = Command::new(&args[2])
        .args(&args[3..])
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on process");
}
