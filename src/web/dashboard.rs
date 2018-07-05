// use super::log;
use client::Client;
use motorsport_calendar_common::event::Event;
use rocket::http::Cookies;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::Template;
use std::collections::HashMap;
use tera::Context;
use web;
use web::WebConfig;

#[derive(Serialize, Debug)]
struct SportInfo {
    pub name: String,
    pub event_count: i32,
    pub session_count: i32,
}

#[get("/dashboard")]
pub fn dashboard(mut cookies: Cookies, config: State<WebConfig>) -> Result<Template, Redirect> {
    web::get_sesssion_from_cookies(&mut cookies)
        .map(|session| {
            debug!("Found session cookie!");
            debug!("Renewing session cookie!");
            let new_session = web::renew_session(&mut cookies, session);
            let mut context = Context::new();
            context.add("username", &new_session.get_user().username);

            let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
            let events = client.get_events().unwrap();
            let info = get_sport_info(&events);
            context.add("sport_info_list", &info);
            Template::render("dashboard", &context)
        })
        .ok_or_else(|| Redirect::to("/login"))
}

fn get_sport_info(events: &[Event]) -> Vec<SportInfo> {
    let mut vec = Vec::new();
    let mut map = HashMap::new();
    for event in events {
        let mut vec = map.entry(event.sport.to_string()).or_insert(Vec::new());
        vec.push(event.clone());
    }

    for (sport, events) in &map {
        let session_count = events
            .iter()
            .fold(0, |sum, val| sum + val.sessions.len() as i32);
        let sport_info = SportInfo {
            name: sport.to_string(),
            event_count: events.len() as i32,
            session_count,
        };
        vec.push(sport_info);
    }
    vec
}
