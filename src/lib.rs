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

use rocket::response::content::Html;
use rocket::State;
use juniper::{FieldResult, RootNode};

pub struct Context {
}

impl Context {
    pub fn new() -> Context {
        Context {
        }
    }
}
impl juniper::Context for Context {}

struct Database;
impl Database {
    pub fn new() -> Database {
        Database {}
    }
}

struct DatabaseMutator;
impl DatabaseMutator {
    pub fn new() -> DatabaseMutator {
        DatabaseMutator {}
    }
}

graphql_object!(DatabaseMutator: Context as "Mutator" |&self| {
    description: "Mutation"

});

graphql_object!(Database: Context as "Query" |&self| {
    description: "The root query object of the schema"
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
            Database::new(),
            DatabaseMutator::new(),
        ))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
}
