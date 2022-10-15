mod db;
mod apiv1;

use crate::apiv1::routes::*;
use actix_web::{web, App, HttpServer, middleware};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");


    HttpServer::new(move|| {
        App::new()
        // pass db pool
        .app_data(web::Data::new(pool.clone()))
        // enable logger
        .wrap(middleware::Logger::default())
        .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
        .service(web::scope("/apiv1")
            .route("/show_all", web::get().to(show_all_posts))
            .route("/get_id", web::post().to(show_id))
            .route("/upload_post", web::post().to(upload_post))
            .route("/update_post", web::post().to(update_a_post))
            .route("/delete_post", web::post().to(delete_a_post))
            
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await


}
