use super::models::{InputTicket, Ticket};
use super::schema::tickets::dsl::*;
use super::Pool;
use actix_web::{web, Error, HttpResponse};
use diesel::{
    dsl::{delete, insert_into},
    QueryDsl, RunQueryDsl,
};
use std::vec::Vec;

// Handler for GET /tickets
pub async fn get_tickets(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_tickets(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for GET /tickets/{id}
pub async fn get_ticket_by_id(
    db: web::Data<Pool>,
    get_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || db_get_ticket_by_id(db, get_id.into_inner()))
            .await
            .map(|result| HttpResponse::Ok().json(result))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

// Handler for POST /tickets
pub async fn add_ticket(
    db: web::Data<Pool>,
    item: web::Json<InputTicket>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || add_db_ticket(db, item))
        .await
        .map(|result| HttpResponse::Created().json(result))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

// Handler for DELETE /tickets/{id}
pub async fn delete_ticket(
    db: web::Data<Pool>,
    get_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || delete_db_ticket(db, get_id.into_inner()))
            .await
            .map(|result| HttpResponse::Ok().json(result))
            .map_err(|_| HttpResponse::InternalServerError())?,
    )
}

fn db_get_ticket_by_id(
    pool: web::Data<Pool>,
    get_id: i32,
) -> Result<Ticket, diesel::result::Error> {
    let conn = pool.get().unwrap();
    tickets.find(get_id).get_result::<Ticket>(&conn)
}

fn get_all_tickets(pool: web::Data<Pool>) -> Result<Vec<Ticket>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = tickets.load::<Ticket>(&conn)?;
    Ok(items)
}

fn add_db_ticket(
    db: web::Data<Pool>,
    item: web::Json<InputTicket>,
) -> Result<Ticket, diesel::result::Error> {
    let conn = db.get().unwrap();
    let new_tickets = Ticket::new_ticket(item.id, &item.owner);
    let res = insert_into(tickets).values(&new_tickets).get_result(&conn)?;
    println!("Successfull add tickets to database!");
    Ok(res)
}

fn delete_db_ticket(db: web::Data<Pool>, get_id: i32) -> Result<usize, diesel::result::Error> {
    let conn = db.get().unwrap();
    let count_id = delete(tickets.find(get_id)).execute(&conn)?;
    Ok(count_id)
}
