mod git_config;
mod util;
use clap::{App, AppSettings, Arg};
use std::io;

use crate::git_config::GitUser;

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

    let git_user = git_config::GitUserCollection::new();
    match matches.subcommand() {
        Some(("list", _arg)) => {
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
            git_user
                .set(
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
            let group_name = arg.value_of("GROUP NAME").unwrap();
            git_user.delete(group_name);
        }

        _ => println!("Please use the help command to see the available commands"),
    }
    Ok(())
}
