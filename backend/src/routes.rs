use actix_web::web;
use crate::handlers::{upload_file, register_user, login_user, get_user, delete_user};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/upload").route(web::post().to(upload_file)))
       .service(web::resource("/register").route(web::post().to(register_user)))
       .service(web::resource("/login").route(web::post().to(login_user)))
       .service(web::resource("/user/{id}").route(web::get().to(get_user)).route(web::delete().to(delete_user)));
}
