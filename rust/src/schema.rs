// @generated automatically by Diesel CLI.

diesel::table! {
    lessons (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        #[max_length = 50]
        category -> Varchar,
        #[max_length = 50]
        topic -> Varchar,
        manual_date -> Nullable<Timestamptz>,
        bookmarked -> Bool,
        assignee_id -> Uuid,
    }
}

diesel::table! {
    tasks (id) {
        id -> Text,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        priority -> Int2,
        completed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        due_date -> Nullable<Timestamptz>,
        #[max_length = 255]
        file -> Nullable<Varchar>,
        assignee_id -> Uuid,
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
        first_name -> Nullable<Varchar>,
        #[max_length = 150]
        last_name -> Nullable<Varchar>,
        #[max_length = 254]
        email -> Nullable<Varchar>,
        is_staff -> Bool,
        is_active -> Bool,
        date_joined -> Timestamptz,
    }
}

diesel::joinable!(lessons -> users (assignee_id));
diesel::joinable!(tasks -> users (assignee_id));

diesel::allow_tables_to_appear_in_same_query!(
    lessons,
    tasks,
    users,
);
