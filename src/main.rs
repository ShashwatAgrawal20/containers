mod core;

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
