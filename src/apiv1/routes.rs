use crate::db::models::Post;
use actix_web::{web, HttpResponse, Result, Error};
use diesel::SqliteConnection;
use crate::apiv1::apimodels::{JsonId, JsonPost, JsonUpdatePost};
use crate::db::view::*;
use crate::db::write::write_to_db;
use crate::db::update::update_post;
use crate::db::delete::delete_post;

use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


/// Returns all posts
///
/// # Arguments
///
/// * `post` - Post (from db schema)
///
/// * `connection` - Database Connection 
pub async fn show_all_posts(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {

    let post_res: Option<Vec<Post>> = web::block(move || {
        let mut conn = pool.get()?;
        find_all_users(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(post_res) = post_res {
        Ok(HttpResponse::Ok().json(post_res))
    } else {
        let res: HttpResponse = HttpResponse::NotFound().body(format!("No posts"));
        Ok(res)
    }
}

/// Returns a post route for showing first id
///
/// # Arguments
///
/// * `post` - Post (from db schema)
///
/// * `connection` - Database Connection 
pub async fn show_id(data: web::Json<JsonId>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {

    let id:i32 = data.id; 

    let post_res: Option<Post> = web::block(move || {
        let mut conn = pool.get()?;
        find_user_by_id(id, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(post_res) = post_res {
        Ok(HttpResponse::Ok().json(post_res))
    } else {
        let res: HttpResponse = HttpResponse::NotFound().body(format!("No post found with id: {id}"));
        Ok(res)
    }

}

/// Uploads a post
///
/// # Arguments
///
/// * `post` - Post (from db schema)
///
/// * `connection` - Database Connection 
pub async fn upload_post(data: web::Json<JsonPost>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {

    let new_title = data.title.clone();
    let new_body = data.body.clone();

    let post_res = web::block(move || {
        let mut conn = pool.get()?;
        write_to_db(new_title, new_body, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(post_res))

}

/// Uploads a post
///
/// # Arguments
///
/// * `post` - Post (from db schema)
///
/// * `connection` - Database Connection 
pub async fn update_a_post(data: web::Json<JsonUpdatePost>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {

    let new_id = data.id.clone();
    let new_title = data.title.clone();
    let new_body = data.body.clone();

    let post_res = web::block(move || {
        let mut conn = pool.get()?;
        update_post(new_id, new_title, new_body, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(post_res) = post_res {
        Ok(HttpResponse::Ok().json(post_res))
    } else {
        let res: HttpResponse = HttpResponse::NotFound().body(format!("No post updated"));
        Ok(res)
    }
}

/// Delete a post
///
/// # Arguments
///
/// * `post` - Post (from db schema)
///
/// * `connection` - Database Connection 
pub async fn delete_a_post(data: web::Json<JsonId>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {

    let new_id = data.id.clone();

    let post_res = web::block(move || {
        let mut conn = pool.get()?;
        delete_post(new_id, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(post_res) = post_res {
        Ok(HttpResponse::Ok().json(post_res))
    } else {
        let res: HttpResponse = HttpResponse::NotFound().body(format!("No post deleted"));
        Ok(res)
    }
}
