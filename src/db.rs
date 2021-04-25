use std::ops::Deref;
use dotenv::dotenv;
use std::env;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure};
use rocket::Request;
use rocket::http::Status;

// Statically initialize our DB pool
lazy_static! {
  pub static ref DB_POOL: r2d2::Pool<ConnectionManager<SqliteConnection>> = {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder().max_size(32).build(manager).expect("failed to create pool")
  };
}

pub struct DB(r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

impl Deref for DB {
  type Target = SqliteConnection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
  type Error = r2d2::Error;
  fn from_request(_: &'a Request<'r>) -> Outcome<Self, Self::Error> {
    match DB_POOL.get() {
      Ok(conn) => Success(DB(conn)),
      Err(e) => Failure((Status::InternalServerError, e)),
    }
  }
}