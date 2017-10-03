use schema::todos;
use rocket_contrib::Json;

#[derive(Queryable, Serialize)]
pub struct Todo {
  pub id: i32,
  pub title: Option<String>,
  pub body: String
}


#[derive(Insertable, Serialize, Deserialize,  Debug)]
#[table_name="todos"]
pub struct NewTodo {
  pub title: Option<String>,
//  pub body: String
}
