use diesel::prelude::*;


table!{
    users {
        id -> Uuid,
        auth_provider_id -> Varchar,
        email -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}