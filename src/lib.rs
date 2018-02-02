
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn get_version() -> &str {
    VERSION
}
