use nix::sched::{unshare, CloneFlags};
use nix::unistd::sethostname;
use std::process::{exit, Command};

pub fn run(args: &Vec<String>) {
    //! For now this function just takes the args Vector and try to run the `command` in that.
    //! **NOTE:** Before running the command it also tries to create a testing `UTS namespace` and set the
    //! hostname to `container`.
    println!("running {:?}", &args[2..]);

    // ----------------- Some shitty NAMESPACES testing -----------------
    if let Err(err) = unshare(CloneFlags::CLONE_NEWUTS) {
        eprintln!("Failed to create UTS namespace: {}", err);
        exit(1);
    }
    if let Err(err) = sethostname("container") {
        eprintln!("Failed to set hostname: {}", err);
        exit(1);
    }
    // ----------------- Some shitty NAMESPACES testing -----------------

    let _output = Command::new(&args[2])
        .args(&args[3..])
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on process");
}
