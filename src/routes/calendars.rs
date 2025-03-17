use actix_web::web;
use crate::handlers::calendars::{list_calendars, get_calendar_by_id, get_calendar_by_name, create_calendar, update_calendar, delete_calendar};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/calendars")
            .route("", web::get().to(list_calendars))
            .route("/{id}", web::get().to(get_calendar_by_id))
            .route("/name/{name}", web::get().to(get_calendar_by_name))
            .route("", web::post().to(create_calendar))
            .route("/{id}", web::put().to(update_calendar))
            .route("/{id}", web::delete().to(delete_calendar))
    );
}