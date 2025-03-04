use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    id: Uuid,
    auth_provider_id: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(auth_provider_id: String, email: String, first_name: Option<String>, last_name:
    Option<String>)
        -> Self {
        Self {
            id: Uuid::new_v4(),
            auth_provider_id,
            email,
            first_name,
            last_name,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn auth_provider_id(&self) -> &str {
        &self.auth_provider_id
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn first_name(&self) -> Option<&str> {
        self.first_name.as_deref()
    }
    pub fn last_name(&self) -> Option<&str> {
        self.last_name.as_deref()
    }
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn update(
        &mut self,
        first_name: Option<String>,
        last_name: Option<String>,
        is_active: bool
    ) {
        if let Some(first_name) = first_name { self.first_name = Some(first_name); }
        if let Some(last_name) = last_name { self.last_name = Some(last_name); }
        self.is_active = is_active;
        self.updated_at = Utc::now();
    }

    pub fn initialize(
        id: Uuid,
        auth_provider_id: String,
        email: String,
        first_name: Option<String>,
        last_name: Option<String>,
        is_active: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            auth_provider_id,
            email,
            first_name,
            last_name,
            is_active,
            created_at,
            updated_at,
        }
    }
}
