use crate::actix::{Actor, Handler, Message, SyncContext};
use crate::model::Msg;
use crate::schema;

use actix_web::error::Error;
use chrono::Local;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde_derive::Deserialize;
use uuid::Uuid;

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

#[derive(Debug, Deserialize)]
pub struct User {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct Form {
    pub username: String,
    pub body: String,
}


impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl Message for Form {
    type Result = Result<Msg, Error>;
}

impl Message for User {
    type Result = Result<Vec<Msg>, Error>;
}

impl Handler<User> for DbActor {
    type Result = Result<Vec<Msg>, Error>;

    fn handle(&mut self, _msg: User, _: &mut Self::Context) -> Self::Result {

        let conn: &PgConnection = &self.0.get().unwrap();

        let msgs = schema::messages::table
            .order(schema::messages::ts.desc())
            .limit(20)
            .load::<Msg>(conn)
            .unwrap();

        Ok(msgs)
    }
}

impl Handler<Form> for DbActor {
    type Result = Result<Msg, Error>;

    fn handle(&mut self, msg: Form, _: &mut Self::Context) -> Self::Result {

        let conn: &PgConnection = &self.0.get().unwrap();

        let new_msg = Msg {
            id: Uuid::new_v4(),
            username: msg.username,
            body: msg.body,
            ts: Local::now().naive_local()
        };

        let result = diesel::insert_into(schema::messages::table)
            .values(&new_msg)
            .get_result(conn)
            .unwrap();

        Ok(result)
    }
}