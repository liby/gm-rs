pub mod config {
    use std::path::{Path, PathBuf};

    use configparser::ini::Ini;

    struct Opts {
        cwd: PathBuf,
        opt_type: String,
    }
    pub fn get() -> std::io::Result<PathBuf> {
        let opts = Opts {
            cwd: std::env::current_dir()?,
            opt_type: String::from("global"),
        };

        let mut config_path = if opts.opt_type == "global" {
            Path::new(&dirs::home_dir().unwrap()).join(".gitconfig")
        } else {
            let mut path = PathBuf::from(opts.cwd);
            path.push(".git/config");
            path
        };

        if !config_path.exists() {
            if opts.opt_type == "global" {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No git config found",
                ));
            }
            config_path = Path::new(&dirs::home_dir().unwrap()).join(".config/git/config")
        }

        match config_path.exists() {
            true => Ok(config_path),
            false => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No git config found",
            )),
        }
    }

    pub fn parse_config() -> std::io::Result<()> {
        let mut config = Ini::new();
        let path = get()?;
        let map = config.load(path);
        println!("{:#?}", map);
        Ok(())
    }
}
