//
// graphql.rs
//

use chrono::NaiveDate;
use juniper::FieldResult;
use uuid::Uuid;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
pub struct Query;
pub struct Mutation;

use kv::*;

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

graphql_object!(Query: Store |&self| {
    field todos(&executor) -> FieldResult<Vec<Todo>> {
        let bucket = executor.context().bucket::<&str, Json<Todo>>(None)?;
        let x: Vec<Todo> = bucket.iter().map(|i| i.unwrap().value::<Json<Todo>>().unwrap().0).collect();

        Ok(x)
    }
});

graphql_object!(Mutation: Store |&self| {
    field insertTodo(&executor, input: TodoInput) -> FieldResult<bool> {
        let bucket = executor.context().bucket::<&str, Json<Todo>>(None)?;

        for i in 0..1000 {
            let uuid = Uuid::new_v4();
            let x = Todo {id: uuid.to_string(), title: input.title.clone(), due_date: Some(input.due_date)};
            bucket.set(uuid.to_string().as_str(), Json(x.clone()))?;
        }
        Ok(true)
    }

});
