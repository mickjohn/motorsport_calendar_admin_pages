use client::Client;
use model::{NewEvent, NewSession};
use rocket::http::Cookies;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_contrib::Template;
use tera::Context;
use web;
use web::{SessionStoreArc, WebConfig};

#[get("/events/create_event")]
pub fn get_new_event_page(
    mut cookies: Cookies,
    session_store: State<SessionStoreArc>,
    flash: Option<FlashMessage>,
) -> Result<Template, Redirect> {
    let mut session_store = session_store.write().unwrap();
    web::get_sesssion_from_cookies(&mut cookies, &session_store)
        .map(|session| {
            let new_session = web::renew_session(&mut cookies, &mut session_store, session);
            let mut context = Context::new();
            context.insert("username", &new_session.get_user().username);
            if let Some(flash_message) = flash {
                context.insert("flash", flash_message.msg());
            }
            Template::render("new_event", &context)
        })
        .ok_or_else(|| Redirect::to("/login"))
}

#[post("/events", data = "<event>")]
pub fn create_event(
    mut cookies: Cookies,
    event: Form<NewEvent>,
    config: State<WebConfig>,
    session_store: State<SessionStoreArc>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let new_event = event.into_inner();
    let mut session_store = session_store.write().unwrap();
    if let Some(session) = web::get_sesssion_from_cookies(&mut cookies, &session_store) {
        let new_session = web::renew_session(&mut cookies, &mut session_store, session);
        // Need to drop cookies before calling the redirect, else there will be
        // multiple cookie instances active at once, and newer instances will be empty!
        // NOT WORKING
        drop(cookies);
        let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
        client.create_event(&new_event).map_err(|e| {
            Flash::error(
                Redirect::to("/500_error.html"),
                format!("Error updating event!\n{}", e),
            )
        })?;
        Ok(Flash::success(
            Redirect::to(&format!("/events")),
            "Event successfully created!",
        ))
    } else {
        Err(Flash::error(
            Redirect::to("/500_error.html"),
            "Error creating event!".to_string(),
        ))
    }
}

#[get("/events/<event_id>/create_session")]
pub fn get_new_session_page(
    mut cookies: Cookies,
    event_id: i32,
    session_store: State<SessionStoreArc>,
    flash: Option<FlashMessage>,
) -> Result<Template, Redirect> {
    let mut session_store = session_store.write().unwrap();
    web::get_sesssion_from_cookies(&mut cookies, &session_store)
        .map(|session| {
            let new_session = web::renew_session(&mut cookies, &mut session_store, session);
            let mut context = Context::new();
            context.insert("username", &new_session.get_user().username);
            context.insert("event_id", &event_id);
            if let Some(flash_message) = flash {
                context.insert("flash", flash_message.msg());
            }
            Template::render("new_session", &context)
        })
        .ok_or_else(|| Redirect::to("/login"))
}

#[post("/events/<event_id>/create_session", data = "<session>")]
pub fn create_session(
    mut cookies: Cookies,
    event_id: i32,
    session: Form<NewSession>,
    config: State<WebConfig>,
    session_store: State<SessionStoreArc>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let new_session = session.into_inner();
    let mut session_store = session_store.write().unwrap();
    if let Some(web_session) = web::get_sesssion_from_cookies(&mut cookies, &session_store) {
        let new_web_session = web::renew_session(&mut cookies, &mut session_store, web_session);
        // Need to drop cookies before calling the redirect, else there will be
        // multiple cookie instances active at once, and newer instances will be empty!
        // NOT WORKING
        drop(cookies);
        let client = Client::new(config.api_url.clone(), new_web_session.get_user().clone());
        client.create_session(&new_session, event_id).map_err(|e| {
            Flash::error(
                Redirect::to("/500_error.html"),
                format!("Error updating event!\n{}", e),
            )
        })?;
        Ok(Flash::success(
            Redirect::to(&format!("/events/{}", event_id)),
            "Session successfully created!",
        ))
    } else {
        Err(Flash::error(
            Redirect::to("/500_error.html"),
            "Error creating session!".to_string(),
        ))
    }
}
