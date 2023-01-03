use crate::schema::tickets;
use diesel::{Insertable, Queryable};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Deserialize, Serialize, Insertable)]
#[table_name = "tickets"]
pub struct Ticket {
    pub id: i32,
    pub count: i32,
    pub owner: String,
}

impl Ticket {
    pub fn new_ticket(id: i32, owner: &str) -> Self {
        let count = rand::thread_rng().gen();
        Self {
            id,
            count,
            owner: owner.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputTicket {
    pub id: i32,
    pub owner: String,
}
