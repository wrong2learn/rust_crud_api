use crate::custom_response::api_response::ApiResponse;
use crate::db::fake_db::FakeDb;
use crate::models::user::User;
use actix_web::{web, HttpResponse, Responder};
/*
pub async fn get_users1() -> impl Responder {
    let mut user_list: Vec<User> = Vec::new();
    let users_to_add = vec![
        User {
            id: 1,
            name: "John Doe".to_string(),
            email: "john.doe@emai.com".to_string(),
        },
        User {
            id: 2,
            name: "Jane Doe".to_string(),
            email: "jane.doe@emai.com".to_string(),
        },
    ];
    user_list.extend(users_to_add);

    HttpResponse::Ok().json(ApiResponse::success(user_list))
}
*/
pub async fn get_users(db: web::Data<FakeDb>) -> impl Responder {
    let users = db.get_users();
    HttpResponse::Ok().json(ApiResponse::success(users))
}

pub async fn add_user(user: web::Json<User>, db: web::Data<FakeDb>) -> impl Responder {
    db.add_user(user.into_inner());
    HttpResponse::Ok().json(ApiResponse::<()>::success(()))
}

pub async fn get_user_by_id(path: web::Path<i32>, db: web::Data<FakeDb>) -> impl Responder {
    let user = db.get_user_by_id(*path);
    match user {
        Some(user) => HttpResponse::Ok().json(ApiResponse::success(user)),
        None => HttpResponse::NotFound().json(ApiResponse::<()>::error("Utente non trovato")),
    }
}

pub async fn delete_user_by_id(path: web::Path<i32>, db: web::Data<FakeDb>) -> impl Responder {
    let user = db.delete_user_by_id(*path);
    match user {
        Some(user) => HttpResponse::Ok().json(ApiResponse::success(user)),
        None => HttpResponse::NotFound().json(ApiResponse::<()>::error("Utente non trovato")),
    }
}

pub async fn cause_error() -> impl Responder {
    // Creiamo una risposta di errore
    HttpResponse::BadRequest().json(ApiResponse::<()>::error("Qualcosa è andato storto"))
}
