mod git_config;
use clap::{App, AppSettings, Arg};

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
                .arg(Arg::new("GROUP NAME").takes_value(true).required(true))
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
            App::new("use")
                .about("Use one group name for user config")
                .arg(
                    Arg::new("GROUP NAME")
                        .takes_value(true)
                        .conflicts_with("global"),
                )
                .arg(
                    Arg::new("global")
                        .long("global")
                        .takes_value(false)
                        .help("Git global config"),
                ),
        )
        .subcommand(
            App::new("delete")
                .about("Delete one group")
                .arg(Arg::new("GROUP NAME").takes_value(true).required(true)),
        )
        .get_matches();

    let config = git_config::GitUserCollection::new();
    match matches.subcommand() {
        Some(("list", _arg)) => {
            config.list();
        }

        Some(("set", _arg)) => {}

        Some(("use", _arg)) => {}

        Some(("delete", _arg)) => {}

        _ => println!("Please use the help command to see the available commands"),
    }
    Ok(())
}
