//
// web.rs
//
pub mod cors;

use crate::db;
use crate::graphql::{Mutation, Query, Schema};
use kv::Store;
use rocket::response::content;
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
    store: State<Store>,
) -> juniper_rocket::GraphQLResponse {
    // Create new context

    request.execute(&schema, store.inner())
}

pub fn launch() {
    let cfg = db::config();
    let store = Store::new(cfg).expect("Could not create store");

    rocket::ignite()
        .manage(store)
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            routes![graphiql, post_graphql_handler, post_graphql_cors_handler],
        )
        .attach(cors::CORS())
        .launch();
}
