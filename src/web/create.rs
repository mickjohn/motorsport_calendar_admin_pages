use super::login;
use client::Client;
use model::{NewEvent, NewSession};
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_contrib::templates::Template;
use session::Session;
use tera::Context;
use web::{static_routes, WebConfig};

#[get("/create/event")]
pub fn get_new_event_page(session: Session, flash: Option<FlashMessage>) -> Template {
    let mut context = Context::new();
    context.insert("username", &session.get_user().username);
    if let Some(flash_message) = flash {
        context.insert("flash", flash_message.msg());
    }
    Template::render("new_event", &context)
}

#[get("/create/event", rank = 2)]
pub fn get_new_event_page_redirect() -> Redirect {
    Redirect::to(uri!(login::login_page))
}

#[post("/events", data = "<event>")]
pub fn create_event(
    event: Form<NewEvent>,
    config: State<WebConfig>,
    session: Session,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let new_event = event.into_inner();
    let client = Client::new(config.api_url.clone(), session.get_user().clone());
    client.create_event(&new_event).map_err(|e| {
        Flash::error(
            Redirect::to(uri!(super::static_routes::internal_server_error)),
            format!("Error updating event!\n{}", e),
        )
    })?;
    Ok(Flash::success(
        Redirect::to(uri!(super::events::get_events)),
        "Event successfully created!",
    ))
}

#[post("/events/<event_id>/create_session", data = "<session>")]
pub fn create_session(
    event_id: i32,
    session: Form<NewSession>,
    config: State<WebConfig>,
    web_session: Session,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let new_session = session.into_inner();
    let client = Client::new(config.api_url.clone(), web_session.get_user().clone());
    client.create_session(&new_session, event_id).map_err(|e| {
        Flash::error(
            Redirect::to(uri!(static_routes::internal_server_error)),
            format!("Error updating event!\n{}", e),
        )
    })?;
    Ok(Flash::success(
        Redirect::to(uri!(super::events::get_event: event_id = event_id)),
        "Session successfully created!",
    ))
}
