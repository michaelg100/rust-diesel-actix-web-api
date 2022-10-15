use crate::db::models::{Post};
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Updates existing post
///
/// # Arguments
///
/// * `post` - Post (from db schema)
///
/// * `connection` - Database Connection 
pub fn update_post(
    post_id: i32, 
    post_title: String, 
    post_body: String, 
    connection: &mut SqliteConnection) -> Result<Option<usize>, DbError> {
    use crate::db::schema::posts::dsl::*;

    let new_post = Post{
        id: post_id,
        title: post_title,
        body: post_body,
    };

    let update = diesel::update(posts.find(post_id))
        .set(&new_post)
        .execute(connection)
        .optional()?;

    Ok(update)
}