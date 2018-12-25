use client::Client;
use rocket::response::{Flash, Redirect};
use rocket::State;
use session::Session;
use web::WebConfig;

#[get("/delete/events/<event_id>")]
pub fn delete_event(
    event_id: i32,
    config: State<WebConfig>,
    session: Session,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let client = Client::new(config.api_url.clone(), session.get_user().clone());
    client.delete_event(event_id).map_err(|e| {
        Flash::error(
            Redirect::to(uri!(super::static_routes::internal_server_error)),
            format!("Error deleting event!\n{}", e),
        )
    })?;
    Ok(Flash::success(
        Redirect::to(uri!(super::events::get_events)),
        "Event successfully deleted!",
    ))
}

#[get("/delete/events/<event_id>/sessions/<session_id>")]
pub fn delete_session(
    event_id: i32,
    session_id: i32,
    config: State<WebConfig>,
    session: Session,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let client = Client::new(config.api_url.clone(), session.get_user().clone());
    client.delete_session(session_id, event_id).map_err(|e| {
        Flash::error(
            Redirect::to(uri!(super::static_routes::internal_server_error)),
            format!("Error deleting event!\n{}", e),
        )
    })?;
    Ok(Flash::success(
        Redirect::to(uri!(super::events::get_event: event_id = event_id)),
        "Event successfully deleted!",
    ))
}
