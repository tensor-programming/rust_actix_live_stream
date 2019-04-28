use crate::actix::prelude::{Addr, Message};
use crate::schema::messages;
use crate::actor::{db::DbActor, ws::WsActor};

use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AppState {
    pub db: Addr<DbActor>,
    pub ws: Addr<WsActor>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Message)]
#[table_name= "messages" ]
pub struct Msg {
    pub id: Uuid,
    pub username: String,
    pub body: String,
    pub ts: NaiveDateTime,
}