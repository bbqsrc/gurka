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
    token: Option<String>
}

impl Context {
    pub fn new() -> Context {
        Context {
            token: None
        }
    }

    pub fn with_auth(mut self, token: String) -> Context {
        self.token = Some(token);
        return self
    }
}
impl juniper::Context for Context {}

struct Database {
    pub pool: PgPool   
}

impl Database {
    pub fn new(pool: PgPool) -> Database {
        Database {
            pool: pool
        }
    }
}

pub trait GurkaMutator {
    fn create_user(&self, username: &str, password: String) -> FieldResult<graphql::models::User>;
    fn log_in(&self, username: &str, password: &str) -> FieldResult<graphql::models::UserSession>;
}

pub struct DatabaseMutator {
    pub pool: PgPool   
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

graphql_object!(DatabaseMutator: Context as "Mutator" |&self| {
    description: "Mutation"

    field create_user(&executor, username: String, password: String) -> FieldResult<graphql::models::User> {
        self.create_user(&username, password)
    }

    field log_in(&executor, username: String, password: String) -> FieldResult<graphql::models::UserSession> {
        self.log_in(&username, &password)
    }
});

graphql_object!(Database: Context as "Query" |&self| {
    description: "The root query object of the schema"

    field user(&executor, username: String) -> FieldResult<Option<graphql::models::User>> {
        Ok(None)
    }
});

type Schema = RootNode<'static, Database, DatabaseMutator>;

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
            Database::new(establish_pool()),
            DatabaseMutator::new(establish_pool()),
        ))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
}
