use configparser::ini::Ini;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::process::Command;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    TableBuilder, TableStyle,
};

pub struct GitUser {
    pub name: String,
    pub email: String,
    pub scope: String,
}
pub struct GitUserCollection {
    data: Vec<GitUser>,
}

impl GitUserCollection {
    pub fn new() -> GitUserCollection {
        let global_config = Self::get_global_config().unwrap();
        let global_sections = global_config.sections();
        let mut config_vec = vec![GitUser {
            name: global_config.get("user", "name").unwrap(),
            email: global_config.get("user", "email").unwrap(),
            scope: String::from("global"),
        }];

        let includeif_sections = global_sections
            .iter()
            .filter(|section| section.starts_with("includeif"))
            .collect::<Vec<&String>>();

        for section in includeif_sections {
            let mut other_config = Ini::new();
            let path_str = global_config.get(section, "path").unwrap();
            let path = expand_tilde(&path_str).unwrap();

            other_config.load(path).unwrap();
            config_vec.push(GitUser {
                name: other_config.get("user", "name").unwrap(),
                email: other_config.get("user", "email").unwrap(),
                scope: section.to_owned(),
            });
        }

        GitUserCollection { data: config_vec }
    }

    pub fn list(self) {
        let mut table_rows = vec![Row::new(vec![
            TableCell::new_with_alignment("Scope", 1, Alignment::Center),
            TableCell::new_with_alignment("Name", 1, Alignment::Center),
            TableCell::new_with_alignment("Email", 1, Alignment::Center),
        ])];

        for user in self.data {
            table_rows.push(Row::new(vec![
                TableCell::new(&user.scope),
                TableCell::new_with_alignment(&user.name, 1, Alignment::Center),
                TableCell::new(&user.email),
            ]));
        }

        let table = TableBuilder::new()
            .style(TableStyle::extended())
            .rows(table_rows)
            .build();

        let git_user_name = Command::new("git")
            .arg("config")
            .arg("user.name")
            .output()
            .expect("failed to execute process");
        let git_user_email = Command::new("git")
            .arg("config")
            .arg("user.email")
            .output()
            .expect("failed to execute process");

        print!(
            "Currently used name: {}, email: {}",
            String::from_utf8_lossy(&git_user_name.stdout).trim(),
            String::from_utf8_lossy(&git_user_email.stdout),
        );

        println!("{}", table.render());
    }

    pub fn get_global_config() -> Result<Ini> {
        let config_path = Path::new(&dirs::home_dir().unwrap()).join(".gitconfig");

        if !config_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No git config found",
            ));
        }

        let mut global_config = Ini::new();
        match config_path.exists() {
            true => {
                let _load = global_config.load(config_path);
                Ok(global_config)
            }
            false => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No git config found",
            )),
        }
    }
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
