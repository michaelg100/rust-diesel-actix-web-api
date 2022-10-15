use crate::db::models::{Post};
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;


/// Run query using Diesel to find user by id and return it.
pub fn find_user_by_id(
    id_check: i32,
    connection: &mut SqliteConnection,
) -> Result<Option<Post>, DbError> {

    use crate::db::schema::posts::dsl::*;

    let post = posts
        .find(id_check)
        .get_result::<Post>(connection)
        .optional()?;

    Ok(post)
}

/// Run query using Diesel to find user by uid and return it.
pub fn find_all_users(
    connection: &mut SqliteConnection,
) -> Result<Option<Vec<Post>>, DbError> {

    use crate::db::schema::posts::dsl::*;

    let post = posts
        .load::<Post>(connection)
        .optional()?;

    Ok(post)
}