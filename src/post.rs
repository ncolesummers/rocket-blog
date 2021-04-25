use diesel::result::Error;
use diesel;
use diesel::sqlite::SqliteConnection;
use crate::models::*;
use diesel::prelude::*;
use crate::schema::posts;

// Returns post with given id
pub fn get_post(conn: &SqliteConnection, id: i32) -> Result<Post, Error> {
  posts::table
    .find(id)
    .first::<Post>(conn)
}

// Returns all posts
pub fn get_posts(conn: &SqliteConnection) -> Result<Vec<Post>, Error> {
  posts::table
    .load::<Post>(conn)
}

// Creates a post with the given PostData, assigns a ID
pub fn create_post(conn: &SqliteConnection, post: PostData) -> bool {
  diesel::insert_into(posts::table).values(&post).execute(conn).is_ok()
}

// Deletes a post with the given ID
pub fn delete_post(conn: &SqliteConnection, id: i32) -> Result<usize, Error> {
  diesel::delete(posts::table.find(id))
    .execute(conn)
}

// Updates a post with the given ID and PostData
pub fn update_post(conn: &SqliteConnection, id: i32, updated_post: PostData) -> bool {
  diesel::update(posts::table
    .find(id))
    .set(&updated_post).execute(conn).is_ok() 
}