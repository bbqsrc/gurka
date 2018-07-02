#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(trivial_bounds)]

extern crate base64;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate derive_builder;
extern crate dotenv;
#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
#[macro_use]
extern crate juniper_relay;
extern crate rocket;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate ring;
extern crate uuid;

use rocket::response::content::Html;
use rocket::State;
use juniper::{FieldResult, FieldError, RootNode};

use r2d2_diesel::ConnectionManager;
use r2d2::Pool;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod schema;
pub mod models;
pub mod graphql;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_pool() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}

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

    pub fn current_user(&self) -> Option<&models::User> {
        match &self.current_user {
            Some(v) => Some(&v),
            None => None
        }
    }
}
impl juniper::Context for Context {}

pub struct QueryHolder;
pub struct MutatorHolder;

pub trait GurkaMutator : Clone + Send + Sync {
    fn create_user(&self, username: &str, password: String) -> FieldResult<graphql::models::User>;
    fn create_project(&self, slug: String, name: String, owner: &models::User) -> FieldResult<graphql::models::Project>;
    fn log_in(&self, username: &str, password: &str) -> FieldResult<graphql::models::UserSession>;
}

#[derive(Clone)]
pub struct DatabaseMutator {
    pool: PgPool
}

impl DatabaseMutator {
    pub fn new(pool: PgPool) -> DatabaseMutator {
        DatabaseMutator {
            pool: pool
        }
    }
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

type Schema = RootNode<'static, QueryHolder, MutatorHolder>;

#[get("/")]
fn graphiql() -> Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

pub fn make_server() -> rocket::Rocket {
    rocket::ignite()
        .manage(Context::new())
        .manage(Schema::new(
            QueryHolder,
            MutatorHolder,
        ))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
}
