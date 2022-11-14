// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "token_type"))]
    pub struct TokenType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TokenType;

    tokens (id) {
        id -> Uuid,
        token_type -> TokenType,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Text,
    }
}

diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tokens,
    users,
);
