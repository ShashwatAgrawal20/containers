use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|x| x.as_str()) {
        Some("run") => run(&args),
        _ => println!("bad command"),
    }
}

fn run(args: &Vec<String>) {
    println!("running {:?}", &args[2..]);
    let _output = Command::new(&args[2])
        .args(&args[3..])
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on process");

    // TODO: Create some shitty NAMESPACES
}
