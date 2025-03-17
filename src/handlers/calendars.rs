use actix_web::{HttpResponse, Responder, web};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{postgres::Postgres, redis::Redis, schema};

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::calendars)]
struct PartialCalendar {
    id: i32,
    name: String,
}

#[derive(Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::calendars)]
struct CompleteCalendar {
    id: i32,
    name: String,
    description: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

#[derive(Deserialize)]
pub struct NewCalendarRequest {
    name: String,
    description: String,
}

#[derive(Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::calendars)]
struct NewCalendar {
    name: String,
    description: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct ModifyCalendarRequest {
    name: String,
    description: String,
}

#[derive(Serialize, AsChangeset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::calendars)]
struct ModifyCalendar {
    name: String,
    description: String,
    updated_at: NaiveDateTime,
}

pub async fn list_calendars(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::calendars::table
        .select(PartialCalendar::as_select())
        .load::<PartialCalendar>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(calendars)) => {
            return HttpResponse::Ok().json(calendars);
        }
        Ok(None) => {
            return HttpResponse::Ok().json(Vec::<PartialCalendar>::new());
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_calendar_by_id(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut __redis_connection = redis.get_connection();

    let result = schema::calendars::table
        .select(CompleteCalendar::as_select())
        .filter(schema::calendars::id.eq(search_id.into_inner()))
        .first::<CompleteCalendar>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(calendar)) => {
            return HttpResponse::Ok().json(calendar);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn get_calendar_by_name(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    search_name: web::Path<String>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let result = schema::calendars::table
        .select(CompleteCalendar::as_select())
        .filter(schema::calendars::name.eq(&search_name.into_inner()))
        .first::<CompleteCalendar>(&mut postgres_connection)
        .optional();

    match result {
        Ok(Some(calendar)) => {
            return HttpResponse::Ok().json(calendar);
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn create_calendar(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    create_calendar_request: web::Json<NewCalendarRequest>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let create_calendar = NewCalendar {
        name: create_calendar_request.name.clone(),
        description: create_calendar_request.description.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let count = diesel::insert_into(schema::calendars::table)
        .values(&create_calendar)
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Created().finish();
        }
        Ok(None) => {
            return HttpResponse::InternalServerError().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn update_calendar(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    update_id: web::Path<i32>,
    update_calendar_request: web::Json<ModifyCalendarRequest>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut _redis_connection = redis.get_connection();

    let update_calendar = ModifyCalendar {
        name: update_calendar_request.name.clone(),
        description: update_calendar_request.description.clone(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    let count = diesel::update(schema::calendars::table)
        .set(&update_calendar)
        .filter(schema::calendars::id.eq(update_id.into_inner()))
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Ok().finish();
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}

pub async fn delete_calendar(
    postgres: web::Data<Postgres>,
    redis: web::Data<Redis>,
    delete_id: web::Path<i32>,
) -> impl Responder {
    let mut postgres_connection = postgres.get_connection();
    let mut __redis_connection = redis.get_connection();

    let count = diesel::delete(schema::calendars::table)
        .filter(schema::calendars::id.eq(delete_id.into_inner()))
        .returning((
            schema::calendars::id,
            schema::calendars::name,
            schema::calendars::description,
        ))
        .execute(&mut postgres_connection)
        .optional();

    match count {
        Ok(Some(_)) => {
            return HttpResponse::Ok().finish();
        }
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(error.to_string());
        }
    }
}
