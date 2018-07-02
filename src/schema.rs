table! {
    projects (id) {
        id -> Int4,
        slug -> Text,
        name -> Text,
        owner_id -> Int4,
    }
}

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

joinable!(projects -> users (owner_id));
joinable!(user_sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    projects,
    user_sessions,
    users,
);
