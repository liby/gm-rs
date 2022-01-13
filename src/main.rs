mod git_config;

pub use crate::git_config::config;

use clap::{App, AppSettings, Arg};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    TableBuilder, TableStyle,
};

fn main() -> Result<(), std::io::Error> {
    let mut table_rows = vec![Row::new(vec![
        TableCell::new_with_alignment("Scope", 1, Alignment::Center),
        TableCell::new_with_alignment("Name", 1, Alignment::Center),
        TableCell::new_with_alignment("Email", 1, Alignment::Center),
    ])];

    for user in config::parse_config().unwrap() {
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

    match matches.subcommand() {
        Some(("list", _arg)) => {
            println!("{}", table.render());
        }

        Some(("set", _arg)) => {}

        Some(("use", _arg)) => {}

        Some(("delete", _arg)) => {}

        _ => println!("Please use the help command to see the available commands"),
    }
    Ok(())
}
