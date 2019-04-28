extern crate actix;
#[macro_use]
extern crate diesel;

mod actor;
mod handler;
mod model;
mod schema;

use crate::actor::{db::DbActor, ws::WsActor};

use crate::handler::{get_ws, login, send_msg};
use crate::model::AppState;
use actix::prelude::*;
use actix_web::{http::Method, server, App};
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;

use std::collections::HashMap;
use std::env;


fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set!");

    let sys = actix::System::new("chat_system");

    let manager = ConnectionManager::<PgConnection>::new(db_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build the Pool");

    let db_addr: Addr<DbActor> = SyncArbiter::start(5, move || DbActor(pool.clone()));
    let ws_addr: Addr<WsActor> = Arbiter::start(|_| WsActor(HashMap::new()));

    server::new(move || {
        App::with_state(AppState {
            db: db_addr.clone(),
            ws: ws_addr.clone(),
        })
        .resource("/login", |r| r.method(Method::POST).with(login))
        .resource("/get_ws", |r| r.method(Method::GET).f(get_ws))
        .resource("/send", |r| r.method(Method::POST).with(send_msg))
    })
    .bind("localhost:8080")
    .unwrap()
    .start();

    let _ = sys.run();
}
