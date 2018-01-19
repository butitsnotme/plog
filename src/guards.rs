extern crate r2d2;
extern crate r2d2_sqlite;

use self::r2d2_sqlite::SqliteConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use rusqlite::Connection;
use std::ops::Deref;

type Pool = r2d2::Pool<SqliteConnectionManager>;
pub struct DbConn(pub r2d2::PooledConnection<SqliteConnectionManager>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
	type Error = ();
	
	fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
		let pool = request.guard::<State<Pool>>()?;
		match pool.get() {
			Ok(conn) => Outcome::Success(DbConn(conn)),
			Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
		}
	}
}

impl Deref for DbConn {
  type Target = Connection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub fn init_pool() -> Pool {
	let manager = SqliteConnectionManager::file("plog.db");
	r2d2::Pool::builder()
             .max_size(20)
             .build(manager)
             .expect("Database connection failed")
}
