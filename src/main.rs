mod core;

/// The main entry point of the program.
///
/// This function collects command-line arguments and matches the first argument to determine
/// which function to call. If the first argument is "run", it calls the `run` function to set
/// up namespaces and execute a child process. If the first argument is "child", it calls the
/// `child` function to execute a command within the current namespace. If the first argument
/// doesn't match any of these options, it prints "bad command".
///
///
/// I am aware of a shitty bug that exists, which is the user can directly invoke the
/// command with the `child` as the argument insted of `run` and will skip the invokation of
/// the `child` process and other `namespace` creation which is a big nono.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|x| x.as_str()) {
        Some("run") => core::run(&args),
        Some("child") => core::child(&args),
        _ => println!("bad command"),
    }
}
