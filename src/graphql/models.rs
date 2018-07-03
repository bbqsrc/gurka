use juniper::{FieldResult, FieldError};

use models;
use context::Context;

#[derive(GraphQLObject)]
pub struct Success {
    is_success: bool
}

pub struct User {
    pub model: models::User
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

relay_connection!(UserConnection, UserEdge, User, Context);

pub struct UserSession {
    pub user: User,
    pub session_model: models::UserSession
}

impl UserSession {
    pub fn new(user: User, session: models::UserSession) -> UserSession {
        UserSession {
            user: user,
            session_model: session
        }
    }

    pub fn token(&self) -> String {
        self.session_model.id.hyphenated().to_string()
    }
}

graphql_object!(UserSession: Context as "UserSession" |&self| {
    description: "A user session"

    field user() -> &User {
        &self.user
    }

    field token() -> String {
        self.token()
    }
});

relay_connection!(UserSessionConnection, UserSessionEdge, UserSession, Context);

pub struct Project {
    pub model: models::Project
}

impl Project {
    pub fn new(project: models::Project) -> Project {
        Project { model: project }
    }
}

graphql_object!(Project: Context as "Project" |&self| {
    description: "A project"

    field slug() -> &str {
        &self.model.slug
    }

    field name() -> &str {
        &self.model.name
    }

    field owner(&executor) -> FieldResult<User> {
        let maybe_user = executor.context().query.user_by_id(self.model.owner_id)?;
        match maybe_user {
            Some(user) => Ok(user),
            None => Err(FieldError::new(
                "No user found for owner_id of project",
                graphql_value!({ "error": "internal error" })
            ))
        }
    }
});

pub struct Feature {
    pub model: models::Feature
}

impl Feature {
    pub fn new(feature: models::Feature) -> Feature {
        Feature { model: feature }
    }
}

graphql_object!(Feature: Context as "Feature" |&self| {
    description: "A feature"

    field slug() -> &str {
        &self.model.slug
    }

    field name() -> &str {
        &self.model.name
    }

    field project(&executor) -> FieldResult<Project> {
        let maybe_project = executor.context().query.project_by_id(self.model.project_id)?;
        match maybe_project {
            Some(project) => Ok(project),
            None => Err(FieldError::new(
                "No project found for project_id of feature",
                graphql_value!({ "error": "internal error" })
            ))
        }
    }
});
