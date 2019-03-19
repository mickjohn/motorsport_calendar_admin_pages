use client::Client;
use model::{NewEvent, NewSession};
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
use rocket::State;
use session::Session;
use web::{static_routes, WebConfig};

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
