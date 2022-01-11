mod git_config;

use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    TableBuilder, TableStyle,
};

pub use crate::git_config::config;

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

    println!("{}", table.render());

    Ok(())
}
