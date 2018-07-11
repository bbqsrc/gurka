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

    pub fn feature_by_id(&self, id: i32) -> FieldResult<Option<graphql::models::Feature>> {
        let db = self.pool.get()?;
        let record = models::Feature::find_by_id(&*db, id)?;
        match record {
            Some(feature) => Ok(Some(graphql::models::Feature::new(feature))),
            None => Ok(None)
        }
    }

    #[doc(hidden)]
    pub fn project_features(&self, project_id: i32) -> FieldResult<Vec<graphql::models::Feature>> {
        let db = self.pool.get()?;
        let maybe_project = models::Project::find_by_id(&*db, project_id)?;
        let project = match maybe_project {
            Some(v) => v,
            None => return Ok(vec![])
        };
        let records = models::Feature::all_by_project(&*db, &project)?;
        Ok(records.into_iter().map(|r| graphql::models::Feature::new(r)).collect())
    }

    #[doc(hidden)]
    pub fn feature_steps(&self, feature_id: i32) -> FieldResult<Vec<graphql::models::Step>> {
        let db = self.pool.get()?;
        let maybe_feature = models::Feature::find_by_id(&*db, feature_id)?;
        let feature = match maybe_feature {
            Some(v) => v,
            None => return Ok(vec![])
        };
        let records = models::Step::all_by_feature(&*db, &feature)?;
        Ok(records.into_iter().map(|r| graphql::models::Step::new(r)).collect())
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