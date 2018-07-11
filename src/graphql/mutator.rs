use ::PgPool;
use juniper::{FieldResult, FieldError};
use context::Context;

use models;
use graphql;

pub struct MutatorHolder;

#[derive(Clone)]
pub struct DatabaseMutator {
    pool: PgPool
}

impl DatabaseMutator {
    pub fn new(pool: PgPool) -> DatabaseMutator {
        DatabaseMutator { pool: pool }
    }
}

pub trait GurkaMutator {
    fn create_user(&self, username: &str, password: String) -> FieldResult<graphql::models::User>;
    fn create_project(&self, slug: String, name: String, owner: &models::User) -> FieldResult<graphql::models::Project>;
    fn create_feature(&self, slug: String, name: String, project: &models::Project, creator: &models::User) -> FieldResult<graphql::models::Feature>;
    fn create_step(&self, step: models::NewStep) -> FieldResult<graphql::models::Step>;
    fn reorder_step_before(&self, src_step: models::Step, target_step: &models::Step) -> FieldResult<Vec<graphql::models::Step>>;
    fn log_in(&self, username: &str, password: &str) -> FieldResult<graphql::models::UserSession>;
    fn delete_project(&self, project: models::Project) -> FieldResult<String>;
    fn rename_project_slug(&self, project: models::Project, new_slug: &str) -> FieldResult<graphql::models::Project>;
}

impl GurkaMutator for DatabaseMutator {
    fn create_user(&self, username: &str, password: String) -> FieldResult<graphql::models::User> {
        let db = self.pool.get()?;
        let record = models::NewUser::create(&*db, username, password)?;
        Ok(graphql::models::User::new(record))
    }

    fn create_project(&self, slug: String, name: String, owner: &models::User) -> FieldResult<graphql::models::Project> {
        let db = self.pool.get()?;
        let record = models::Project::new(&*db, models::NewProject {
            slug: slug,
            name: name,
            owner_id: owner.id
        })?;
        Ok(graphql::models::Project::new(record))
    }

    fn create_feature(&self, slug: String, name: String, project: &models::Project, creator: &models::User) -> FieldResult<graphql::models::Feature> {
        let db = self.pool.get()?;
        let record = models::Feature::new(&*db, models::NewFeature {
            project_id: project.id,
            creator_id: creator.id,
            slug: slug,
            name: name
        })?;
        Ok(graphql::models::Feature::new(record))
    }

    fn create_step(&self, new_step: models::NewStep) -> FieldResult<graphql::models::Step> {
        let db = self.pool.get()?;
        let record = models::Step::new(&*db, new_step)?;
        Ok(graphql::models::Step::new(record))
    }

    fn reorder_step_before(&self, src_step: models::Step, target_step: &models::Step) -> FieldResult<Vec<graphql::models::Step>> {
        let db = self.pool.get()?;
        let records = models::Step::move_to(&*db, src_step, target_step.position)?;
        Ok(records.into_iter().map(|r| graphql::models::Step::new(r)).collect())
    }

    fn delete_project(&self, project: models::Project) -> FieldResult<String> {
        let db = self.pool.get()?;
        let id = models::Project::delete(&*db, project)?;
        Ok(id)
    }

    fn rename_project_slug(&self, project: models::Project, new_slug: &str) -> FieldResult<graphql::models::Project> {
        let db = self.pool.get()?;
        let record = models::Project::rename_slug(&*db, project, new_slug)?;
        Ok(graphql::models::Project::new(record))
    }

    fn log_in(&self, username: &str, password: &str) -> FieldResult<graphql::models::UserSession> {
        let db = self.pool.get()?;
        let maybe_user = models::User::find_by_username(&*db, username)?;
        let user = match maybe_user {
            Some(v) => v,
            None => return Err(FieldError::new(
                "No user found or invalid password",
                graphql_value!({ "error": "no user found" })
            ))
        };
        if !user.verify_password(password) {
            return Err(FieldError::new(
                "No user found or invalid password",
                graphql_value!({ "error": "no user found" })
            ))
        }
        let session = models::UserSession::create(&*db, user.id)?;
        Ok(graphql::models::UserSession::new(graphql::models::User::new(user), session))
    }
}

graphql_object!(MutatorHolder: Context as "Mutator" |&self| {
    description: "Mutation"

    field create_user(&executor, username: String, password: String) -> FieldResult<graphql::models::User> {
        executor.context().mutator.create_user(&username, password)
    }

    field create_project(&executor, slug: String, name: String) -> FieldResult<graphql::models::Project> {
        match executor.context().current_user() {
            Some(user) => {
                executor.context().mutator.create_project(slug, name, &user)
            },
            None => Err(FieldError::new(
                "No user associated with this session",
                graphql_value!({ "error": "forbidden" })
            ))
        }
    }

    field log_in(&executor, username: String, password: String) -> FieldResult<graphql::models::UserSession> {
        executor.context().mutator.log_in(&username, &password)
    }
});