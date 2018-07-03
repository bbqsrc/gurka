use rocket::response::content::Html;
use rocket::{self, State};
use juniper::RootNode;
use juniper_rocket::{GraphQLRequest, GraphQLResponse, graphiql_source};

use context::Context;
use graphql::{QueryHolder, MutatorHolder};

type Schema = RootNode<'static, QueryHolder, MutatorHolder>;

#[get("/")]
fn graphiql() -> Html<String> {
    graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: GraphQLRequest,
    schema: State<Schema>,
) -> GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: GraphQLRequest,
    schema: State<Schema>,
) -> GraphQLResponse {
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
