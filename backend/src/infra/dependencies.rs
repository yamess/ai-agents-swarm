use crate::infra::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config
}
impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config
        }
    }
}