use sqlx::sqlite::SqlitePool;
use std::env;
use structopt::StructOpt;

mod async_query;
mod generate;
mod metric;
mod stream;
mod transaction;

use async_query::async_query;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Add,
}

const QUERY: &'static str = r#"
SELECT id, area, age
FROM user"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::from_args_safe()?;

    match args.cmd {
        Some(Command::Add) => {
            let qt: i64 = 10_000_000;
            println!("adding {} entries", qt);
            generate::gen(qt);
            println!("done");
        }
        None => {
            let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

            async_query(&QUERY, &pool).await?;
        }
    }
    Ok(())
}
