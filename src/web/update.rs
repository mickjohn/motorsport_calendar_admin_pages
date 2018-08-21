use client::Client;
use model::*;
use rocket::http::Cookies;
use rocket::http::Status;
use rocket::response::status;
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket_contrib::Json;
use web;
use web::{SessionStoreArc, WebConfig};

/*
 * This class contains the route that is used to update the event, it's sessions, and it's new
 * sessions. It's not an idomatic REST endpoint, but it has to be this way to a more user friendly
 * admin page, one that has the event, sessions and new sessions on the one page/form.
 *
 * Also, to get the redirects to work correctly the whole thing has to be spoofed with a form,
 * instead of XML http request, and the XML http request doesn't follow redirects, and if you
 * manually redirect then the FlashCookie is unset.
 */

// The struct to hold all of the new and updates sessions/events.
#[derive(Debug, Deserialize)]
pub struct Update {
    updated_event: EventUpdate,
    updated_sessions: Vec<SessionUpdate>,
    new_sessions: Vec<NewSession>,
}

// #[post("/update_events_and_sessions/<event_id>", format = "application/json", data = "<update>")]
// fn update_events_and_sessions(
//     mut cookies: Cookies,
//     update: Json<Update>,
//     event_id: i32,
//     config: State<WebConfig>,
//     session_store: State<SessionStoreArc>,
// ) -> Result<Redirect, Flash<Redirect>> {
//     let mut session_store = session_store.write().unwrap();
//     if let Some(session) = web::get_sesssion_from_cookies(&mut cookies, &session_store) {
//         let new_session = web::renew_session(&mut cookies, &mut session_store, session);
//         let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
//         validate(&client, &update, event_id)?;
//         update_event(&client, &update.updated_event, event_id)?;
//         update_sessions(&client, &update.updated_sessions, event_id)?;
//         create_new_sessions(&client, &update.new_sessions, event_id)?;
//         Ok(Redirect::to(&format!("/events/{}", &event_id)))
//     } else {
//         Err(Flash::error(Redirect::to("/"), ""))
//     }
// }

#[post("/validate_selections/<event_id>", format = "application/json", data = "<update>")]
fn validate_selections(
    mut cookies: Cookies,
    update: Json<Update>,
    event_id: i32,
    config: State<WebConfig>,
    session_store: State<SessionStoreArc>,
) -> Result<status::Custom<String>, status::Custom<String>> {
    let mut session_store = session_store.write().unwrap();
    if let Some(session) = web::get_sesssion_from_cookies(&mut cookies, &session_store) {
        let new_session = web::renew_session(&mut cookies, &mut session_store, session);
        let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
        let event = client.get_event(event_id).unwrap();
        let session_ids: Vec<&i32> = event.sessions.iter().map(|s| &s.id).collect();
        // Verify that the session actually belongs to this event.
        for session in &update.updated_sessions {
            if !session_ids.contains(&&session.id) {
                warn!(
                    "Session ID does not belong. Session ids: {:?}, session id: {}",
                    session_ids, session.id
                );
                return Err(status::Custom(
                    Status::InternalServerError,
                    "Session does not belong to event!".to_string(),
                ));
            }
        }
        Ok(status::Custom(Status::Ok, "Hi!".to_string()))
    } else {
        Err(status::Custom(
            Status::InternalServerError,
            "Error!".to_string(),
        ))
    }
}

#[post("/update_event/<event_id>", format = "application/json", data = "<event>")]
fn update_event(
    mut cookies: Cookies,
    event: Json<EventUpdate>,
    event_id: i32,
    config: State<WebConfig>,
    session_store: State<SessionStoreArc>,
) -> Result<status::Custom<String>, status::Custom<String>> {
    let mut session_store = session_store.write().unwrap();
    if let Some(session) = web::get_sesssion_from_cookies(&mut cookies, &session_store) {
        let new_session = web::renew_session(&mut cookies, &mut session_store, session);
        let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
        client.update_event(&event, event_id).map_err(|_| {
            status::Custom(
                Status::InternalServerError,
                "Error updating event!".to_string(),
            )
        })?;
        Ok(status::Custom(Status::Ok, "Hi!".to_string()))
    } else {
        Err(status::Custom(
            Status::InternalServerError,
            "Error!".to_string(),
        ))
    }
}

#[post("/update_session/<event_id>", format = "application/json", data = "<session_update>")]
fn update_session(
    mut cookies: Cookies,
    session_update: Json<SessionUpdate>,
    event_id: i32,
    config: State<WebConfig>,
    session_store: State<SessionStoreArc>,
) -> Result<status::Custom<String>, status::Custom<String>> {
    let mut session_store = session_store.write().unwrap();
    if let Some(session) = web::get_sesssion_from_cookies(&mut cookies, &session_store) {
        let new_session = web::renew_session(&mut cookies, &mut session_store, session);
        let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
        client
            .update_session(session_update.clone(), event_id)
            .map_err(|_| {
                status::Custom(
                    Status::InternalServerError,
                    "Error updating session!".to_string(),
                )
            })?;
        Ok(status::Custom(Status::Ok, "Hi!".to_string()))
    } else {
        Err(status::Custom(
            Status::InternalServerError,
            "Error!".to_string(),
        ))
    }
}

/*
 * validate selections
 * update event
 * for session...
 *  update session
 * for new session...
 *  create session
 */
// fn validate_selections()
// fn validate_session()
// fn update_event()
// fn update_session()
// fn create_session()

fn validate_session(s: &SessionUpdate) -> Result<(), Flash<Redirect>> {
    if s.name.len() >= 256 {
        return Err(Flash::error(
            Redirect::to("/500_error.html"),
            "Session name cannot be longer than 256 characters.",
        ));
    }
    Ok(())
}

// fn update_event(
//     client: &Client,
//     event: &EventUpdate,
//     event_id: i32,
// ) -> Result<(), Flash<Redirect>> {
//     client.update_event(event, event_id).map_err(|_| {
//         Flash::error(
//             Redirect::to("/500_error.html"),
//             "Internal server error updating event...",
//         )
//     })?;
//     Ok(())
// }

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
