use crate::actix::prelude::*;
use crate::actix::{Actor, Handler, Running, StreamHandler};
use crate::actor::ws::{Connect, Disconnect};
use crate::model::{AppState, Msg};
use actix_web::ws::{Message, ProtocolError, WebsocketContext};
use std::time::{Duration, Instant};
use uuid::Uuid;

pub struct Session(pub Uuid, pub Instant);

impl Actor for Session {
    type Context = WebsocketContext<Self, AppState>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);

        let id = Uuid::new_v4();
        self.0 = id;

        let addr = ctx.address().recipient();

        ctx.state().ws.do_send(Connect { id, addr });
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        ctx.state().ws.do_send(Disconnect { id: self.0 });
        Running::Stop
    }
}

impl Session {
    fn heartbeat(&self, ctx: &mut WebsocketContext<Self, AppState>) {
        ctx.run_interval(Duration::from_secs(5), |actor, ctx| {
            if Instant::now().duration_since(actor.1) > Duration::from_secs(10) {
                println!("Heartbeat failed");

                ctx.state().ws.do_send(Disconnect { id: actor.0 });

                ctx.stop();
            }
        });

        ctx.ping("");
    }
}

impl Handler<Msg> for Session {
    type Result = ();

    fn handle(&mut self, msg: Msg, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

impl StreamHandler<Message, ProtocolError> for Session {
    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        match msg {
            Message::Ping(msg) => {
                self.1 = Instant::now();
                ctx.pong(&msg);
            }
            Message::Pong(_) => {
                self.1 = Instant::now();
            }
            Message::Text(_) => {}
            Message::Binary(_) => {}
            Message::Close(_) => {
                ctx.stop();
            }
        }
    }
}