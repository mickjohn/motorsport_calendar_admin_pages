use client::Client;
use model::*;
use rocket::http::Cookies;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::status;
use rocket::response::{Flash, Redirect};
use rocket::State;
use web;
use web::{SessionStoreArc, WebConfig};

#[post("/events/<event_id>", data = "<event>")]
fn update_event(
    mut cookies: Cookies,
    event: Form<EventUpdate>,
    event_id: i32,
    config: State<WebConfig>,
    session_store: State<SessionStoreArc>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let event_update = event.into_inner();
    let mut session_store = session_store.write().unwrap();
    if let Some(session) = web::get_sesssion_from_cookies(&mut cookies, &session_store) {
        let new_session = web::renew_session(&mut cookies, &mut session_store, session);
        // Need to drop cookies before calling the redirect, else there will be
        // multiple cookie instances active at once, and newer instances will be empty!
        // NOT WORKING
        drop(cookies);
        let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
        client.update_event(&event_update, event_id).map_err(|e| {
            Flash::error(
                Redirect::to("/500_error.html"),
                format!("Error updating event!\n{}", e),
            )
        })?;
        Ok(Flash::success(Redirect::to(
            &format!("/events/{}", event_id)),
            "Event successfully updated!",
        ))
    } else {
        Err(Flash::error(
            Redirect::to("/500_error.html"),
            "Error updating event!".to_string(),
        ))
    }
}

#[post("/update_session/<event_id>", format = "application/json", data = "<session_update>")]
fn update_session(
    mut cookies: Cookies,
    session_update: Form<SessionUpdate>,
    event_id: i32,
    config: State<WebConfig>,
    session_store: State<SessionStoreArc>,
) -> Result<status::Custom<String>, status::Custom<String>> {
    let mut session_store = session_store.write().unwrap();
    if let Some(session) = web::get_sesssion_from_cookies(&mut cookies, &session_store) {
        let new_session = web::renew_session(&mut cookies, &mut session_store, session);
        let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
        client
            .update_session(session_update.into_inner(), event_id)
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

// fn update_sessions(
//     client: &Client,
//     sessions: &[SessionUpdate],
//     event_id: i32,
// ) -> Result<(), Flash<Redirect>> {
//     for session in sessions {
//         client
//             .update_session(session.clone(), event_id)
//             .map_err(|_| {
//                 Flash::error(
//                     Redirect::to("/500_error.html"),
//                     "Internal server error updating session...",
//                 )
//             })?;
//     }
//     Ok(())
// }

// fn create_new_sessions(
//     client: &Client,
//     sessions: &[NewSession],
//     event_id: i32,
// ) -> Result<(), Flash<Redirect>> {
//     for session in sessions {
//         client.create_session(session, event_id).map_err(|_| {
//             Flash::error(
//                 Redirect::to("/500_error.html"),
//                 "Internal server error creating session...",
//             )
//         })?;
//     }
//     Ok(())
// }
