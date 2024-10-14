// @generated automatically by Diesel CLI.

diesel::table! {
    equipments (id) {
        id -> Int4,
        publicid -> Uuid,
        name -> Varchar,
    }
}
