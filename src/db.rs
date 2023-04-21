use std::{env, error::Error};

use diesel::{pg::Pg, r2d2::ConnectionManager, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::{Pool, PooledConnection};

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub async fn establish_connection() -> Result<ConnectionPool, Box<dyn Error + Send + Sync + 'static>>
{
    let pool: ConnectionPool = create_connection_pool();

    run_migrations(&mut pool.get().expect("unable to get DB connection from pool"))
        .expect("Unable to run migrations");

    Ok(pool)
}

fn create_connection_pool() -> ConnectionPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Pool::builder()
        .test_on_check_out(true)
        .build(ConnectionManager::new(database_url))
        .expect("Could not create connection pool")
}

fn run_migrations(
    connection: &mut impl MigrationHarness<Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    print!("Processing pending database migrations... ");
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;
    println!("Done!");

    Ok(())
}
