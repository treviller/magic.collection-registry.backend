// @generated automatically by Diesel CLI.

diesel::table! {
    tokens (id) {
        id -> Uuid,
        token_type -> Varchar,
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
