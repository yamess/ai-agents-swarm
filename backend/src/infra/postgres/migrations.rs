use crate::infra::postgres::connection::{get_connection, DbPool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;
use std::sync::Arc;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations(pool: Arc<DbPool>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut connection = get_connection(pool)?;
    tracing::debug!("Running migrations");
    connection.run_pending_migrations(MIGRATIONS)?;
    tracing::debug!("Migrations complete");
    Ok(())
}
