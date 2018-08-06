use client::Client;
use model::*;
use rocket::http::Cookies;
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_contrib::{Json, Value};
use web;
use web::WebConfig;

/*
 * This class contains the route that is used to update the event, it's sessions, and it's new
 * sessions. It's not an idomatic REST endpoint, but it has to be this way to a more user friendly
 * admin page, one that has the event, sessions and new sessions on the one page/form.
 */

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
    config: State<WebConfig>,
    event_id: i32,
) -> Result<Redirect, Flash<Redirect>> {
    if let Some(session) = web::get_sesssion_from_cookies(&mut cookies) {
        let new_session = web::renew_session(&mut cookies, session);
        let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
        validate(&client, &update, &event_id)?;
        update_event(&client, &update.updated_event, &event_id)?;
        update_sessions(&client, &update.updated_sessions, &event_id)?;
        create_new_sessions(&client, &update.new_sessions, &event_id)?;
        Ok(Redirect::to(&format!("/events/{}", &event_id)))
    } else {
        Err(Flash::error(Redirect::to("/"), ""))
    }
}

fn validate(client: &Client, update: &Update, event_id: &i32) -> Result<(), Flash<Redirect>> {
    let events = client.get_events().unwrap();
    let event = events.get(*event_id as usize).unwrap();
    let session_ids: Vec<&i32> = event.sessions.iter().map(|s| &s.id).collect();

    // Verify that the session actually belongs to this event.
    for session in &update.updated_sessions {
        if !session_ids.contains(&&session.id) {
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
    event_id: &i32,
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
    event_id: &i32,
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
    event_id: &i32,
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
