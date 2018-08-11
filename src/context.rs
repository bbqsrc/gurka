use juniper;

use ::{PgPool, establish_pool};
use graphql::{DatabaseMutator, DatabaseQuery};
use models;

#[derive(Clone)]
pub struct Context {
    pub pool: PgPool,
    pub token: Option<String>,
    pub current_user: Option<models::User>,
    pub mutator: DatabaseMutator,
    pub query: DatabaseQuery
}

impl Context {
    pub fn new() -> Context {
        let pool = establish_pool();
        
        Context {
            pool: pool.clone(),
            token: None,
            current_user: None,
            mutator: DatabaseMutator::new(pool.clone()),
            query: DatabaseQuery::new(pool)
        }
    }

    pub fn with_auth(mut self, token: String) -> Context {
        self.token = Some(token);
        return self
    }

    pub fn with_user(mut self, user: Option<models::User>) -> Context {
        self.current_user = user;
        return self
    }

    pub fn current_user(&self) -> Option<&models::User> {
        match &self.current_user {
            Some(v) => Some(&v),
            None => None
        }
    }
}
impl juniper::Context for Context {}