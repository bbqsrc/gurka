use ::PgPool;
use juniper::{FieldResult, FieldError};

use models;
use graphql;
use context::Context;

pub struct QueryHolder;

#[derive(Clone)]
pub struct DatabaseQuery {
    pool: PgPool
}

impl DatabaseQuery {
    pub fn new(pool: PgPool) -> DatabaseQuery {
        DatabaseQuery {
            pool: pool
        }
    }

    pub fn project_by_slug(&self, slug: &str) -> FieldResult<Option<graphql::models::Project>> {
        let db = self.pool.get()?;
        let record = models::Project::find_by_slug(&*db, slug)?;
        match record {
            Some(project) => Ok(Some(graphql::models::Project::new(project))),
            None => Ok(None)
        }
    }

    pub fn project_by_id(&self, id: i32) -> FieldResult<Option<graphql::models::Project>> {
        let db = self.pool.get()?;
        let record = models::Project::find_by_id(&*db, id)?;
        match record {
            Some(project) => Ok(Some(graphql::models::Project::new(project))),
            None => Ok(None)
        }
    }

    pub fn user_by_id(&self, id: i32) -> FieldResult<Option<graphql::models::User>> {
        let db = self.pool.get()?;
        let record = models::User::find_by_id(&*db, id)?;
        match record {
            Some(user) => Ok(Some(graphql::models::User::new(user))),
            None => Ok(None)
        }
    }

    pub fn user_by_username(&self, username: &str) -> FieldResult<Option<graphql::models::User>> {
        let db = self.pool.get()?;
        let record = models::User::find_by_username(&*db, username)?;
        match record {
            Some(user) => Ok(Some(graphql::models::User::new(user))),
            None => Ok(None)
        }
    }

    pub fn feature(&self, project_slug: &str, feature_slug: &str) -> FieldResult<Option<graphql::models::Feature>> {
        let db = self.pool.get()?;
        let record = models::Feature::find_one(&*db, project_slug, feature_slug)?;
        match record {
            Some(feature) => Ok(Some(graphql::models::Feature::new(feature))),
            None => Ok(None)
        }
    }
}

graphql_object!(QueryHolder: Context as "Query" |&self| {
    description: "The root query object of the schema"

    field user(&executor, username: Option<String>, id: Option<i32>) -> FieldResult<Option<graphql::models::User>> {
        if let Some(id) = id {
            return executor.context().query.user_by_id(id);
        }

        if let Some(username) = username {
            return executor.context().query.user_by_username(&username);
        }
        
        return Err(FieldError::new(
            "Either `username` or `id` is required",
            graphql_value!({ "error": "invalid parameters" })
        ))
    }

    field project(&executor, slug: String) -> FieldResult<Option<graphql::models::Project>> {
        executor.context().query.project_by_slug(&slug)
    }
});