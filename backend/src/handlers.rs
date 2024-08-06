use actix_web::{web, HttpRequest, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use crate::models::{User, AppState};

pub async fn upload_file(_req: HttpRequest, _payload: web::Payload) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn register_user(
    state: web::Data<AppState>,
    new_user: web::Json<User>,
) -> impl Responder {
    let mut users = state.users.lock().unwrap();

    if users.iter().any(|u| u.username == new_user.username) {
        return HttpResponse::Conflict().finish();
    }

    let hashed_password = hash(&new_user.password_hash, DEFAULT_COST).unwrap();
    let user = User {
        id: Uuid::new_v4(),
        username: new_user.username.clone(),
        password_hash: hashed_password,
    };

    users.push(user);
    HttpResponse::Created().finish()
}

pub async fn login_user(
    state: web::Data<AppState>,
    credentials: web::Json<User>,
) -> impl Responder {
    let users = state.users.lock().unwrap();

    if let Some(user) = users.iter().find(|u| u.username == credentials.username) {
        if verify(&credentials.password_hash, &user.password_hash).unwrap() {
            return HttpResponse::Ok().finish();
        }
    }

    HttpResponse::Unauthorized().finish()
}

pub async fn get_user(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
) -> impl Responder {
    let users = state.users.lock().unwrap();

    if let Some(user) = users.iter().find(|u| u.id == *user_id) {
        return HttpResponse::Ok().json(user);
    }

    HttpResponse::NotFound().finish()
}

pub async fn delete_user(
    state: web::Data<AppState>,
    user_id: web::Path<Uuid>,
) -> impl Responder {
    let mut users = state.users.lock().unwrap();
    if let Some(pos) = users.iter().position(|u| u.id == *user_id) {
        users.remove(pos);
        return HttpResponse::Ok().finish();
    }

    HttpResponse::NotFound().finish()
}
