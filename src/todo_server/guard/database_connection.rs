//extern crate todo_server;
extern crate r2d2_diesel;
extern crate r2d2;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;
use super::super::Pool;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
    let pool = request.guard::<State<Pool>>()?;
    match pool.get() {
      Ok(conn) => Outcome::Success(DbConn(conn)),
      Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
    }
  }
}

// For the convenience of using an &DbConn as an &MysqlConnection.
impl Deref for DbConn {
  type Target = MysqlConnection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
