// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        title -> Varchar,
        price -> Float4,
        image -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        is_admin -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    products,
    users,
);
