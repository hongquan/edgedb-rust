use async_std::task;

use edgeql_parser::helpers::quote_name;
use crate::options::{Options, Command, CreateCommand};
use crate::client::Connection;


pub fn main(options: Options) -> Result<(), anyhow::Error> {
    match options.subcommand.as_ref().expect("subcommand is present") {
        Command::Create(w) => match &w.subcommand {
            CreateCommand::Database(d) => {
                task::block_on(async {
                    let mut conn = Connection::from_options(&options).await?;
                    let mut cli = conn.authenticate(&options).await?;
                    let res = cli.execute(&format!("CREATE DATABASE {}",
                                         quote_name(&d.database_name))).await?;
                    eprintln!("  -> {}: Ok",
                        String::from_utf8_lossy(&res[..]));
                    Ok(())
                }).into()
            }
        },
        Command::Alter => todo!(),
        Command::Configure => todo!(),
        Command::Drop => todo!(),
    }
}
