use client::Client;
use motorsport_calendar_common::event::Event;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;
use session::Session;
use std::collections::HashMap;
use tera::Context;
use web::WebConfig;
use web::login;

#[derive(Serialize, Debug)]
struct SportInfo {
    pub name: String,
    pub event_count: i32,
    pub session_count: i32,
}

#[get("/dashboard")]
pub fn dashboard(config: State<WebConfig>, session: Session) -> Template {
    let mut context = Context::new();
    context.insert("username", &session.get_user().username);

    let client = Client::new(config.api_url.clone(), session.get_user().clone());
    let events = client.get_events().unwrap();
    let info = get_sport_info(&events);
    context.insert("sport_info_list", &info);
    Template::render("dashboard", &context)
}

#[get("/dashboard", rank = 2)]
pub fn dashboard_redirect() -> Redirect {
    Redirect::to(uri!(login::login_page))
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
