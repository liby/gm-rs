mod git_config;

pub use crate::git_config::config;

fn main() -> Result<(), std::io::Error> {
    println!("{:#?}", config::parse_config());
    Ok(())
}
