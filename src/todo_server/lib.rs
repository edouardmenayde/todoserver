#[macro_use]
extern crate serde_derive;

extern crate dotenv;

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate r2d2_diesel;
extern crate r2d2;

use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;

pub mod schema;
pub mod models;
pub mod guard;

// An alias to the type for a pool of Diesel SQLite connections.
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &'static str = env!("DATABASE_URL");

/// Initializes a database pool.
pub fn init_pool() -> Pool {
  let config = r2d2::Config::default();
  let manager = ConnectionManager::<MysqlConnection>::new(DATABASE_URL);
  r2d2::Pool::new(config, manager).expect("db pool")
}
