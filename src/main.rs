mod git_config;
mod util;
use clap::{App, AppSettings, Arg};
use configparser::ini::Ini;
use std::io;
use std::{
    fs::{self},
    io::Write,
};

use crate::git_config::GitUser;
use crate::util::expand_tilde;

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("gum")
        .bin_name("gum")
        .version("0.1.0")
        .about("Git multiple user config manager")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("list").about("List all the user config group"))
        .subcommand(
            App::new("set")
                .about("Set one group for user config")
                .arg(
                    Arg::new("name")
                        .long("name")
                        .takes_value(true)
                        .help("User name"),
                )
                .arg(
                    Arg::new("email")
                        .long("email")
                        .takes_value(true)
                        .help("User email"),
                ),
        )
        .subcommand(
            App::new("delete")
                .about("Delete one group")
                .arg(Arg::new("GROUP NAME").takes_value(true).required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("list", _arg)) => {
            let git_user = git_config::GitUserCollection::new();
            git_user.list();
        }

        Some(("set", sub_matches)) => {
            println!("Please input your setting scope.");

            let mut scope = String::new();

            io::stdin()
                .read_line(&mut scope)
                .expect("Failed to read line");

            println!("You setting scope: {}", scope);

            println!("Please input your config path");
            let mut config_path = String::new();

            io::stdin()
                .read_line(&mut config_path)
                .expect("Failed to read line");

            println!("You config path: {}", config_path);
            git_config::GitUserCollection::set_config(
                GitUser {
                    email: sub_matches.value_of("email").unwrap().to_string(),
                    name: sub_matches.value_of("name").unwrap().to_string(),
                    scope,
                },
                &config_path,
            )
            .unwrap();
        }

        Some(("delete", arg)) => {
            let global_config = git_config::GitUserCollection::get_global_config().unwrap();
            let group_name = arg.value_of("GROUP NAME").unwrap();
            let scope_path = &format!("includeIf \"gitdir:{scope}\"", scope = group_name);
            let scope_config_path = global_config.get(scope_path, "path");
            let mut scope_config = Ini::new();
            scope_config
                .load(expand_tilde(scope_config_path.unwrap()).unwrap())
                .unwrap();
            scope_config.remove_section("user");
        }

        _ => println!("Please use the help command to see the available commands"),
    }
    Ok(())
}
