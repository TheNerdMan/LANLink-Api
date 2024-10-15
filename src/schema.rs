// @generated automatically by Diesel CLI.

diesel::table! {
    equipments (id) {
        id -> Int4,
        publicid -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        publicid -> Uuid,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        discord_username -> Varchar,
        steam_url -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    equipments,
    users,
);
