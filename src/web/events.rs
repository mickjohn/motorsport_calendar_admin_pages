use client::Client;
use rocket::http::Cookies;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::Template;
use tera::Context;
use web;
use web::{SessionStoreArc, WebConfig};

#[derive(FromForm)]
pub struct SportType {
    pub sport_type: String,
}

#[get("/events/<event_id>")]
pub fn get_event(
    mut cookies: Cookies,
    event_id: i32,
    config: State<WebConfig>,
    session_store: State<SessionStoreArc>,
) -> Result<Template, Redirect> {
    let mut session_store = session_store.write().unwrap();
    web::get_sesssion_from_cookies(&mut cookies, &session_store)
        .map(|session| {
            let new_session = web::renew_session(&mut cookies, &mut session_store, session);
            let mut context = Context::new();
            context.add("username", &new_session.get_user().username);

            let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
            let event = client.get_event(event_id).unwrap();
            context.add("event", &event);
            Template::render("event", &context)
        })
        .ok_or_else(|| Redirect::to("/login"))
}

#[get("/events")]
pub fn get_events(
    mut cookies: Cookies,
    config: State<WebConfig>,
    session_store: State<SessionStoreArc>,
) -> Result<Template, Redirect> {
    let mut session_store = session_store.write().unwrap();
    web::get_sesssion_from_cookies(&mut cookies, &session_store)
        .map(|session| {
            let new_session = web::renew_session(&mut cookies, &mut session_store, session);
            let mut context = Context::new();
            context.add("username", &new_session.get_user().username);
            let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
            let events = client.get_events().unwrap();
            context.add("events", &events);
            Template::render("events", &context)
        })
        .ok_or_else(|| Redirect::to("/login"))
}

#[get("/events?<sport_type>")]
pub fn get_events_query(
    mut cookies: Cookies,
    config: State<WebConfig>,
    sport_type: Option<SportType>,
    session_store: State<SessionStoreArc>,
) -> Result<Template, Redirect> {
    let mut session_store = session_store.write().unwrap();
    web::get_sesssion_from_cookies(&mut cookies, &session_store)
        .map(|session| {
            let new_session = web::renew_session(&mut cookies, &mut session_store, session);
            let mut context = Context::new();
            context.add("username", &new_session.get_user().username);

            let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
            let mut events = client.get_events().unwrap();

            if let Some(sport) = sport_type {
                events.retain(|ref event| event.sport == sport.sport_type);
            }

            context.add("events", &events);
            Template::render("events", &context)
        })
        .ok_or_else(|| Redirect::to("/login"))
}

#[get("/events/<event_id>/sessions/<session_id>")]
pub fn get_session(
    mut cookies: Cookies,
    config: State<WebConfig>,
    event_id: i32,
    session_id: i32,
    session_store: State<SessionStoreArc>,
) -> Result<Template, Redirect> {
    let mut session_store = session_store.write().unwrap();
    web::get_sesssion_from_cookies(&mut cookies, &session_store)
        .map(|session| {
            let new_session = web::renew_session(&mut cookies, &mut session_store, session);
            let mut context = Context::new();
            context.add("username", &new_session.get_user().username);

            let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
            let session = client.get_session(event_id, session_id).unwrap();
            context.add("event", &session);
            context.add("session", &session);
            Template::render("session", &context)
        })
        .ok_or_else(|| Redirect::to("/login"))
}
