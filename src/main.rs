#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate todo_server;
extern crate serde_json;
extern crate rocket_contrib;
extern crate diesel;
extern crate rocket;

use serde_json::ser::to_string;
use todo_server::*;
use todo_server::models::*;
use diesel::prelude::*;
use rocket_contrib::json::Json;

use guard::database_connection::*;

#[get("/todo", format = "application/json")]
fn get_todo(conn: DbConn) -> String {
  use todo_server::schema::todos::dsl::*;

  let results = todos.load::<Todo>(&*conn).expect("Error loading posts");

  to_string(&results).expect("Error parsing posts to JSON")
}


#[post("/todo", format = "application/json", data = "<todo>")]
fn post_todo(conn: DbConn, todo: Json<NewTodo>) -> String {
  use diesel;
  use todo_server::schema::todos::dsl::*;

  diesel::insert(&todo).into(todos)
      .execute(&*conn)
      .expect("Error insert post");

  String::from("{}")
  //  to_string(&todo).expect("Error parsing post to JSON")
}

fn main() {
  rocket::ignite()
      .manage(init_pool())
      .mount("/", routes![get_todo])
      .launch();
}
