//
// graphql.rs
//

use chrono::NaiveDate;
use juniper::FieldResult;
use uuid::Uuid;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
pub struct Ctx {}
pub struct Query;
pub struct Mutation;

use kv::*;

fn config() -> Config {
    Config {
        path: "db.sled".into(),
        read_only: false,
        temporary: false,
        use_compression: false,
        flush_every_ms: None,
    }
}

#[derive(Clone, GraphQLObject, serde::Serialize, serde::Deserialize, PartialEq)]
struct Todo {
    id: String,
    title: String,
    due_date: Option<NaiveDate>,
}

#[derive(Clone, GraphQLInputObject, serde::Serialize, serde::Deserialize, PartialEq)]
struct TodoInput {
    title: String,
    due_date: NaiveDate,
}

graphql_object!(Query: Ctx |&self| {
    field todos(&executor) -> FieldResult<Vec<Todo>> {
        let  cfg = config();
        let store = Store::new(cfg)?;
        let bucket = store.bucket::<&str, Json<Todo>>(None)?;
        let x: Vec<Todo> = bucket.iter().map(|i| i.unwrap().value::<Json<Todo>>().unwrap().0).collect();

        Ok(x)
    }
});

graphql_object!(Mutation: Ctx |&self| {
    field insertTodo(&executor, input: TodoInput) -> FieldResult<Todo> {
        let uuid = Uuid::new_v4();
        let  cfg = config();
        let store = Store::new(cfg)?;
        let bucket = store.bucket::<&str, Json<Todo>>(None)?;

        let x = Todo {id: uuid.to_string(), title: input.title, due_date: Some(input.due_date)};
        bucket.set(uuid.to_string().as_str(), Json(x.clone()))?;
        Ok(x)
    }
});
