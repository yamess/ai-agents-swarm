use crate::infra::config::Config;
use crate::infra::postgres::connection::establish_connection;
use crate::infra::postgres::migrations::run_migrations;

#[derive(Clone)]
pub struct AppState {
    pub config: Config
}
impl AppState {
    pub fn new(config: Config) -> Self {
        let db_pool = establish_connection(config.secret().postgres_connection_string());
        run_migrations(db_pool).unwrap();

        Self {
            config
        }
    }
}