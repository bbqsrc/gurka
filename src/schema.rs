table! {
    user_sessions (id) {
        id -> Uuid,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        iterations -> Int4,
        salt -> Bytea,
        credential -> Bytea,
    }
}

joinable!(user_sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    user_sessions,
    users,
);
