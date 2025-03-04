use uuid::Uuid;
use crate::domain::user::model::User;
use crate::prelude::*;


pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<User>;
    async fn get(&self, id: &Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_auth_provider_id(&self, auth_provider_id: &str) -> Result<Option<User>>;
    async fn update(&self, user: &User) -> Result<User>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}