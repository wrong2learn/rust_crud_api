mod controllers {
    pub mod users_controller;
}

mod models {
    pub mod user;
}

mod custom_response {
    pub mod api_response;
}

mod db {
    pub mod fake_db;
}

use actix_web::{web, App, HttpServer};
use controllers::users_controller::{add_user, cause_error, delete_user_by_id, get_user_by_id, get_users};
use db::fake_db::FakeDb;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(FakeDb::new());
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(add_user))
            .route("/users/{id}", web::get().to(get_user_by_id))
            .route("/users/{id}", web::delete().to(delete_user_by_id))
            .route("/error", web::get().to(cause_error))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
