mod core;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|x| x.as_str()) {
        Some("run") => core::run(&args),
        _ => println!("bad command"),
    }
}
