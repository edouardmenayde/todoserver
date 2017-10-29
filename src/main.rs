#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate todo_server;
extern crate serde_json;
extern crate rocket_contrib;
extern crate diesel;
extern crate rocket;

use todo_server::*;
use todo_server::models::*;
use rocket_contrib::Json;

use guard::database_connection::*;

#[get("/", format = "application/json")]
fn get_todo(conn: DbConn) -> Json<Vec<Todo>> {
  let results = Todo::select(&*conn).expect("Error loading posts");

  Json(results)
}


#[post("/", format = "application/json", data = "<todo>")]
fn post_todo(conn: DbConn, todo: Json<NewTodo>) -> Json<Todo> {
  let new_todo = todo.into_inner();

  let todo = Todo::insert(new_todo, &*conn).expect("Error inserting post.");

  Json(todo)
}

#[put("/<id>", format = "application/json", data = "<todo>")]
fn update_todo(conn: DbConn, id: i32, todo: Json<UpdatedTodo>) -> Json<Todo> {
  let updated_todo = todo.into_inner();

  let todo = Todo::update(id, updated_todo, &*conn).expect("Error updating post");

  Json(todo)
}

#[delete("/<id>")]
fn delete_todo(conn: DbConn, id: i32) -> Json<Todo> {
  let todo = Todo::delete(id, &*conn);

  Json(todo)
}

fn main() {
  rocket::ignite()
      .manage(init_pool())
      .mount("/todo", routes![get_todo, post_todo, update_todo, delete_todo])
      .launch();
}
