//
// web.rs
//
pub mod cors;

use crate::graphql::{Ctx, Mutation, Query, Schema};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::response::content;
use rocket::Outcome;
use rocket::State;

#[get("/graphql/explorer")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[options("/graphql")]
fn post_graphql_cors_handler() -> content::Plain<String> {
    content::Plain("".to_string())
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    // Create new context
    let context = Ctx {};

    request.execute(&schema, &context)
}

pub fn launch() {
    rocket::ignite()
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            routes![graphiql, post_graphql_handler, post_graphql_cors_handler],
        )
        .attach(cors::CORS())
        .launch();
}
