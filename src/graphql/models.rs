use juniper::{FieldResult, FieldError};

use models;
use context::Context;

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

    field projects(&executor) -> FieldResult<Vec<Project>> {
       executor.context().query.user_projects(&self.model)
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

    field features(&executor) -> FieldResult<Vec<Feature>> {
        executor.context().query.project_features(self.model.id)
    }
});

pub struct Step {
    pub model: models::Step
}

#[derive(GraphQLInputObject)]
pub struct StepInput {
    pub feature_slug: String,
    pub step_type: String,
    pub value: String,
    pub position: Option<i32>
}

#[derive(GraphQLInputObject)]
pub struct FeatureInput {
    pub project_slug: String,
    pub slug: String,
    pub name: String
}

impl Step {
    pub fn new(step: models::Step) -> Step {
        Step { model: step }
    }
}

graphql_object!(Step: Context as "Step" |&self| {
    description: "A step"

    field type() -> &str {
        &self.model.step_type
    }

    field value() -> &str {
        &self.model.value
    }

    field position() -> i32 {
        self.model.position
    }

    field feature(&executor) -> FieldResult<Feature> {
        let maybe_feature = executor.context().query.feature_by_id(self.model.feature_id)?;
        match maybe_feature {
            Some(feature) => Ok(feature),
            None => Err(FieldError::new(
                "No feature found for feature_id of step",
                graphql_value!({ "error": "internal error" })
            ))
        }
    }

    field creator(&executor) -> FieldResult<User> {
        let maybe_user = executor.context().query.user_by_id(self.model.creator_id)?;
        match maybe_user {
            Some(user) => Ok(user),
            None => Err(FieldError::new(
                "No user found for creator_id of step",
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

    field steps(&executor) -> FieldResult<Vec<Step>> {
        executor.context().query.feature_steps(self.model.id)
    }

    field creator(&executor) -> FieldResult<User> {
        let maybe_user = executor.context().query.user_by_id(self.model.creator_id)?;
        match maybe_user {
            Some(user) => Ok(user),
            None => Err(FieldError::new(
                "No user found for creator_id of step",
                graphql_value!({ "error": "internal error" })
            ))
        }
    }
});
