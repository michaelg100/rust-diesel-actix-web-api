use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Returns a post with the given id
///
/// # Arguments
///
/// * `post` - Post (from db schema)
///
/// * `connection` - Database Connection 
pub fn delete_post(id_check: i32, connection: &mut SqliteConnection) -> Result<Option<usize>, DbError> {
    use crate::db::schema::posts::dsl::*;

    let deleted = diesel::delete(posts.find(id_check))
        .execute(connection)
        .optional()?;

    Ok(deleted)
}