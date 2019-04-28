use crate::actor::db::{Form, User};
use crate::actor::session::Session;
use crate::model::AppState;
use actix_web::{
    error, ws, AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json, State,
};
use futures::Future;
use uuid::Uuid;

pub fn login((user, state): (Json<User>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(user.into_inner())
        .from_err()
        .and_then(|response| match response {
            Ok(msgs) => Ok(HttpResponse::Ok().json(msgs)),
            Err(_) => Ok(HttpResponse::InternalServerError().json("Internal Server Error")),
        })
        .responder()
}

pub fn send_msg((form, state): (Json<Form>, State<AppState>)) -> FutureResponse<HttpResponse> {
    let ws = state.ws.clone();

    state
        .db
        .send(form.into_inner())
        .from_err()
        .and_then(move |response| match response {
            Ok(result) => {
                ws.do_send(result.clone());
                Ok(HttpResponse::Ok().json(result.clone()))
            }
            Err(_) => Ok(HttpResponse::InternalServerError().json("Internal Server Error")),
        })
        .responder()
}

pub fn get_ws(req: &HttpRequest<AppState>) -> Result<HttpResponse, error::Error> {
    ws::start(req, Session(Uuid::nil(), std::time::Instant::now()))
}