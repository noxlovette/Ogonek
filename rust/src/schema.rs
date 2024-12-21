// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 128]
        password -> Varchar,
        last_login -> Nullable<Timestamptz>,
        is_superuser -> Bool,
        #[max_length = 150]
        username -> Varchar,
        #[max_length = 150]
        first_name -> Varchar,
        #[max_length = 150]
        last_name -> Varchar,
        #[max_length = 254]
        email -> Varchar,
        is_staff -> Bool,
        is_active -> Bool,
        date_joined -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
