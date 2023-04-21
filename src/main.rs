mod api;
mod db;
mod models;
mod schema;
mod update;

use crate::{api::server, db::ConnectionPool, update::check};
use std::{env, error::Error, process::ExitCode};

use dotenvy::dotenv;
use tokio::{join, time};

#[tokio::main]
async fn main() -> ExitCode {
    dotenv().ok();

    match inner_main().await {
        Ok(_) => println!("Success!"),
        Err(e) => {
            println!("Error: {}", e);
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}

async fn inner_main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let update_interval: u64 = match env::var("UPDATE_INTERVAL") {
        Ok(val) => val.parse().expect("UPDATE_INTERVAL must be a number"),
        Err(_) => 60 * 10, // 10 minutes
    };

    let pool: ConnectionPool = db::establish_connection()
        .await
        .expect("Unable to establish DB connection");

    let api = server::run_server(&pool);
    let update_checker = check::run(&pool, time::Duration::from_secs(update_interval));

    join!(api, update_checker);
    Ok(())
}
