use nix::sched::{unshare, CloneFlags};
use nix::unistd::sethostname;
use std::process::{exit, Command};

pub fn run(args: &Vec<String>) {
    //! Sets up a new UTS and PID namespace and executes a child process within that namespace.
    //!
    //! This function creates a new UTS and PID namespace using the Linux kernel's `unshare` system
    //! call and sets the hostname to "container". It then spawns a child process using the same
    //! binary (`/proc/self/exe`) with the "child" argument and the provided command and arguments.
    //!
    //! The `child` function is called when the child process is executed, which then runs the
    //! specified command within the namespace.
    //!
    //! # Arguments
    //!
    //! * `args` - A vector of strings representing the command-line arguments passed to the program.
    println!("process id of parent:- {}", std::process::id());

    // ----------------- Some shitty NAMESPACES testing -----------------
    if let Err(err) = unshare(CloneFlags::CLONE_NEWUTS | CloneFlags::CLONE_NEWPID) {
        eprintln!("Failed to create UTS namespace: {}", err);
        exit(1);
    }
    if let Err(err) = sethostname("container") {
        eprintln!("Failed to set hostname: {}", err);
        exit(1);
    }
    // ----------------- Some shitty NAMESPACES testing -----------------

    let _output = Command::new("/proc/self/exe")
        .args(format!("child {} {}", &args[2], &args[3..].join(" ")).split_whitespace())
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on process");
}

pub fn child(args: &Vec<String>) {
    //! Executes a command within the current UTS and PID namespace.
    //!
    //! This shitty function is meant to be called when the `run` function spawn the `child`
    //! process. It takes the command and its arguments and executes it within the newly created
    //! `namespaces`.
    //!
    //! # Arguments
    //!
    //! * `args` - A vector of strings representing the command and its arguments.
    println!("running {:?}", &args[2..]);

    println!("process id in child:- {}", std::process::id());

    let _output = Command::new(&args[2])
        .args(&args[3..])
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on process");
}
