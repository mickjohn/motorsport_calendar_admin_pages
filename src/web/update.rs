use client::Client;
use model::*;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::status;
use rocket::response::{Flash, Redirect};
use rocket::State;
use session::Session;
use web::{static_routes, WebConfig};

#[post("/events/<event_id>", data = "<event>")]
pub fn update_event(
    event: Form<EventUpdate>,
    event_id: i32,
    config: State<WebConfig>,
    session: Session,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let event_update = event.into_inner();
    let client = Client::new(config.api_url.clone(), session.get_user().clone());
    client.update_event(&event_update, event_id).map_err(|e| {
        Flash::error(
            Redirect::to(uri!(static_routes::internal_server_error)),
            format!("Error updating event!\n{}", e),
        )
    })?;
    Ok(Flash::success(
        Redirect::to(uri!(super::events::get_event: event_id = event_id)),
        "Event successfully updated!",
    ))
}

#[post("/update_session/<event_id>", data = "<session_update>")]
pub fn update_session(
    session_update: Form<SessionUpdate>,
    event_id: i32,
    config: State<WebConfig>,
    session: Session,
) -> Result<status::Custom<String>, status::Custom<String>> {
    let client = Client::new(config.api_url.clone(), session.get_user().clone());
    client
        .update_session(session_update.into_inner(), event_id)
        .map_err(|_| {
            status::Custom(
                Status::InternalServerError,
                "Error updating session!".to_string(),
            )
        })?;
    Ok(status::Custom(Status::Ok, "Hi!".to_string()))
}
