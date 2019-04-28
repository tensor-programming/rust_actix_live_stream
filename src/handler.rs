use crate::actor::db::{Form, User};
use crate::actor::session::Session;
use crate::model::AppState;
use actix_web::{
    error, ws, AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json, State,
};