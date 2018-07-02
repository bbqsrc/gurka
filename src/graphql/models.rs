use ::models;
use ::schema;
use ::Context;
use juniper::{FromContext, FieldResult};
use diesel::prelude::*;
use juniper_relay::PageInfo;
use uuid::Uuid;

pub struct User {
    model: models::User
}

impl User {
    pub fn new(user: models::User) -> User {
        User { model: user }
    }
}

graphql_object!(User: Context as "User" |&self| {
    description: "A user"

    field id() -> i32 {
        self.model.id
    }

    field username() -> &String {
        &self.model.username
    }
});

// relay_connection!(UserConnection, UserEdge, User, Context);

pub struct UserSession {
    pub user: User,
    pub session: models::UserSession
}

impl UserSession {
    pub fn new(user: User, session: models::UserSession) -> UserSession {
        UserSession {
            user: user,
            session: session
        }
    }
}

graphql_object!(UserSession: Context as "UserSession" |&self| {
    description: "A user session"

    field user() -> &User {
        &self.user
    }

    field token() -> String {
        self.session.id.hyphenated().to_string()
    }
});


// relay_connection!(UserSessionConnection, UserSessionEdge, UserSession, Context);
