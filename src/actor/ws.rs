use crate::actix::{Actor, Context, Handler, Message, Recipient};
use crate::model::Msg;

use std::collections::HashMap;
use uuid::Uuid;

pub struct WsActor(pub HashMap<Uuid, Recipient<Msg>>);

#[derive(Message)]
pub struct Connect {
    pub id: Uuid,
    pub addr: Recipient<Msg>,
}

#[derive(Message)]
pub struct Disconnect {
    pub id: Uuid,
}

impl Actor for WsActor {
    type Context = Context<WsActor>;
}

impl Handler<Connect> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        self.0.insert(msg.id, msg.addr);
    }
}

impl Handler<Disconnect> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        self.0.remove(&msg.id);
    }
}

impl Handler<Msg> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: Msg, _: &mut Self::Context) {
        for ws in self.0.values() {
            ws.do_send(msg.clone());
        }
    }
}