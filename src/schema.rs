// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "card_rarity"))]
    pub struct CardRarity;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "set_type"))]
    pub struct SetType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "token_type"))]
    pub struct TokenType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CardRarity;

    cards (id) {
        id -> Uuid,
        scryfall_id -> Varchar,
        name -> Varchar,
        lang -> Varchar,
        released_at -> Date,
        rarity -> CardRarity,
        set_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SetType;

    sets (id) {
        id -> Uuid,
        code -> Varchar,
        name -> Varchar,
        set_type -> SetType,
        released_at -> Date,
        block_code -> Nullable<Varchar>,
        block -> Nullable<Varchar>,
        parent_set_id -> Nullable<Uuid>,
        card_count -> Int4,
        printed_size -> Int4,
        foil_only -> Bool,
        non_foil_only -> Bool,
        icon_svg_uri -> Text,
    }
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

diesel::joinable!(cards -> sets (set_id));
diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cards,
    sets,
    tokens,
    users,
);
