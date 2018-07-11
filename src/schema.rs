table! {
    features (id) {
        id -> Int4,
        project_id -> Int4,
        creator_id -> Int4,
        slug -> Text,
        name -> Text,
    }
}

table! {
    projects (id) {
        id -> Int4,
        slug -> Text,
        name -> Text,
        owner_id -> Int4,
    }
}

table! {
    steps (id) {
        id -> Int4,
        slug -> Text,
        feature_id -> Int4,
        creator_id -> Int4,
        step_type -> Text,
        value -> Text,
        position -> Int4,
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

joinable!(features -> projects (project_id));
joinable!(features -> users (creator_id));
joinable!(projects -> users (owner_id));
joinable!(steps -> features (feature_id));
joinable!(steps -> users (creator_id));
joinable!(user_sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    features,
    projects,
    steps,
    user_sessions,
    users,
);
