use client::Client;
use model::*;
use rocket::http::Cookies;
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_contrib::Json;
use web;
use web::{SessionStoreArc, WebConfig};

/*
 * This class contains the route that is used to update the event, it's sessions, and it's new
 * sessions. It's not an idomatic REST endpoint, but it has to be this way to a more user friendly
 * admin page, one that has the event, sessions and new sessions on the one page/form.
 */

// The struct to hold all of the new and updates sessions/events.
#[derive(Debug, Deserialize)]
pub struct Update {
    updated_event: EventUpdate,
    updated_sessions: Vec<SessionUpdate>,
    new_sessions: Vec<NewSession>,
}

#[post("/update_events_and_sessions/<event_id>", format = "application/json", data = "<update>")]
fn update_events_and_sessions(
    mut cookies: Cookies,
    update: Json<Update>,
    event_id: i32,
    config: State<WebConfig>,
    session_store: State<SessionStoreArc>,
) -> Result<Redirect, Flash<Redirect>> {
    let mut session_store = session_store.write().unwrap();
    if let Some(session) = web::get_sesssion_from_cookies(&mut cookies, &session_store) {
        let new_session = web::renew_session(&mut cookies, &mut session_store, session);
        let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
        // validate(&client, &update, event_id)?;
        match validate(&client, &update, event_id) {
            Ok(_) => (),
            Err(_) => return Ok(Redirect::to("/")),
        };
        update_event(&client, &update.updated_event, event_id)?;
        update_sessions(&client, &update.updated_sessions, event_id)?;
        create_new_sessions(&client, &update.new_sessions, event_id)?;
        Ok(Redirect::to(&format!("/events/{}", &event_id)))
    } else {
        Err(Flash::error(Redirect::to("/"), ""))
    }
}

fn validate(client: &Client, update: &Update, event_id: i32) -> Result<(), Flash<Redirect>> {
    let event = client.get_event(event_id).unwrap();
    info!("event id = {}, event = {:#?}", event_id, event);
    let session_ids: Vec<&i32> = event.sessions.iter().map(|s| &s.id).collect();

    // Verify that the session actually belongs to this event.
    for session in &update.updated_sessions {
        if !session_ids.contains(&&session.id) {
            info!("Session ID does not belong. Session ids: {:?}, session id: {}", session_ids, session.id);
            return Err(Flash::error(
                Redirect::to("/500_error.html"),
                "Session does not belong to the event you tried to update.",
            ));
        }
        validate_session(session)?;
    }
    Ok(())
}

fn validate_session(s: &SessionUpdate) -> Result<(), Flash<Redirect>> {
    if s.name.len() >= 256 {
        return Err(Flash::error(
            Redirect::to("/500_error.html"),
            "Session name cannot be longer than 256 characters.",
        ));
    }
    Ok(())
}

fn update_event(
    client: &Client,
    event: &EventUpdate,
    event_id: i32,
) -> Result<(), Flash<Redirect>> {
    client.update_event(event, event_id).map_err(|_| {
        Flash::error(
            Redirect::to("/500_error.html"),
            "Internal server error updating event...",
        )
    })?;
    Ok(())
}

fn update_sessions(
    client: &Client,
    sessions: &[SessionUpdate],
    event_id: i32,
) -> Result<(), Flash<Redirect>> {
    for session in sessions {
        client
            .update_session(session.clone(), event_id)
            .map_err(|_| {
                Flash::error(
                    Redirect::to("/500_error.html"),
                    "Internal server error updating session...",
                )
            })?;
    }
    Ok(())
}
fn create_new_sessions(
    client: &Client,
    sessions: &[NewSession],
    event_id: i32,
) -> Result<(), Flash<Redirect>> {
    for session in sessions {
        client.create_session(session, event_id).map_err(|_| {
            Flash::error(
                Redirect::to("/500_error.html"),
                "Internal server error creating session...",
            )
        })?;
    }
    Ok(())
}
