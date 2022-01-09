pub mod config {
    use configparser::ini::Ini;
    use std::fs;
    use std::path::{Path, PathBuf};

    struct Opts {
        cwd: PathBuf,
        opt_type: String,
    }

    #[derive(Debug)]
    pub struct GitUser {
        name: String,
        email: String,
        scope: String,
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

    pub fn parse_config() -> Result<Vec<GitUser>, Box<dyn std::error::Error>> {
        let mut config = Ini::new();
        config.load(get()?)?;
        let sections = config.sections();
        let includeif_sections = sections
            .iter()
            .filter(|section| section.starts_with("includeif"))
            .collect::<Vec<&String>>();
        let mut other_config_vec = Vec::new();
        for section in includeif_sections {
            let mut other_config = Ini::new();
            let path_str = config.get(section, "path").unwrap();

            let path = expand_tilde(&path_str).unwrap();

            other_config.load(path)?;
            other_config_vec.push(GitUser {
                name: other_config.get("user", "name").unwrap(),
                email: other_config.get("user", "email").unwrap(),
                scope: section.to_owned(),
            });
        }

        Ok(other_config_vec)
    }

    fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
        let p = path_user_input.as_ref();
        if !p.starts_with("~") {
            return Some(p.to_path_buf());
        }
        if p == Path::new("~") {
            return dirs::home_dir();
        }
        dirs::home_dir().map(|mut h| {
            if h == Path::new("/") {
                // Corner case: `h` root directory;
                // don't prepend extra `/`, just drop the tilde.
                p.strip_prefix("~").unwrap().to_path_buf()
            } else {
                h.push(p.strip_prefix("~/").unwrap());
                h
            }
        })
    }
}
