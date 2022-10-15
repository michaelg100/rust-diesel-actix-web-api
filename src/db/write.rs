use crate::db::models::{NewPost, Post};
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Returns a post with the given id
///
/// # Arguments
///
/// * `post` - Post (from db schema)
///
/// * `connection` - Database Connection 
pub fn write_to_db(new_title: String, new_body: String, connection: &mut SqliteConnection) -> Result<Post, DbError> {
    use crate::db::schema::posts::dsl::*;

    let new_post = NewPost{
        title: &new_title,
        body: &new_title
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(connection)
        .unwrap();

    let post= Post {
        id: 0,
        title: new_title,
        body: new_body,
    }; 
        
    Ok(post)

}