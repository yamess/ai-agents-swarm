// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
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
