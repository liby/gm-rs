use crate::util::{expand_tilde, global_config_path};
use configparser::ini::Ini;
use std::io::Result;
use std::process::Command;
use std::{
    fs::{self},
    io::Write,
};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    TableBuilder, TableStyle,
};

#[derive(Debug)]
pub struct GitUser {
    pub name: String,
    pub email: String,
    pub scope: String,
}

#[derive(Debug)]
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
            .filter(|section| section.starts_with("includeIf"))
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

    pub fn get_global_config() -> Result<Ini> {
        let config_path = global_config_path();

        if !config_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No git config found",
            ));
        }

        let mut global_config = Ini::new_cs();

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

    pub fn set(&self, git_user: GitUser, config_path: &str) -> Result<()> {
        let mut config_file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(expand_tilde(&config_path.trim()).unwrap())
            .unwrap();
        let data = format!(
            "
[user]
    email = {email}
    name = {name}",
            email = git_user.email,
            name = git_user.name,
        );
        write!(config_file, "{}", data)?;

        let file_path = global_config_path();
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)
            .unwrap();

        let data = format!(
            "
[includeIf \"gitdir:{scope}\"]
    path = {config_path}",
            scope = git_user.scope.trim(),
            config_path = config_path.trim(),
        );

        write!(file, "{}", data)?;
        Ok(())
    }

    pub fn delete(&self, group_name: &str) {
        let global_config = Self::get_global_config().unwrap();
        let scope_path = &format!("includeIf \"gitdir:{scope}\"", scope = group_name);
        let scope_config_path =
            expand_tilde(global_config.get(scope_path, "path").unwrap()).unwrap();
        let mut scope_config = Ini::new();
        scope_config.load(&scope_config_path).unwrap();
        scope_config.remove_section("user");
        scope_config.write(&scope_config_path).unwrap();
    }
}
