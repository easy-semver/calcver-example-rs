const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("Version v{}", VERSION);
}
