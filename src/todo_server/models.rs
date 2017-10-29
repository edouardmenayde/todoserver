extern crate diesel;

use schema::todos;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

#[derive(Queryable, Serialize)]
pub struct Todo {
  pub id: i32,
  pub title: Option<String>,
  pub body: String
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "todos"]
pub struct NewTodo {
  pub title: Option<String>,
  pub body: String
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "todos"]
pub struct UpdatedTodo {
  pub title: Option<String>,
  pub body: Option<String>
}

impl Todo {
  pub fn insert(todo: NewTodo, conn: &MysqlConnection) -> QueryResult<Todo> {
    use schema::todos::dsl::{todos as all_todos, id};

    diesel::insert(&todo)
        .into(all_todos)
        .execute(&*conn)
        .expect("Could not insert todo.");

    all_todos.order(id.desc()).first(&*conn)
  }

  pub fn select(conn: &MysqlConnection) -> QueryResult<Vec<Todo>> {
    use schema::todos::dsl::todos as all_todos;

    all_todos.load::<Todo>(&*conn)
  }

  pub fn update(id: i32, updated_todo: UpdatedTodo, conn: &MysqlConnection) -> QueryResult<Todo> {
    use schema::todos::dsl::{todos as all_todos, id as _id};

    let _post: Todo = all_todos
        .find(id)
        .first(&*conn)
        .expect("Unable to find post.");

    diesel::update(all_todos)
        .set(&updated_todo)
        .execute(&*conn)
        .expect("Could not update todo.");

    all_todos.order(_id.desc()).first(&*conn)
  }

  pub fn delete(id: i32, conn: &MysqlConnection) -> Todo {
    use schema::todos::dsl::{todos as all_todos, id as _id};

    let post: Todo = all_todos
        .find(id)
        .first(&*conn)
        .expect("Unable to find post.");

    diesel::delete(all_todos.filter(_id.eq(id)))
        .execute(&*conn)
        .expect("Could not update todo.");

    post
  }
}
