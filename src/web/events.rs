use client::Client;
use rocket::http::Cookies;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::Template;
use tera::Context;
use web;
use web::WebConfig;

#[derive(FromForm)]
pub struct SportType {
    pub sport_type: String,
}

#[get("/events")]
pub fn get_events(mut cookies: Cookies, config: State<WebConfig>) -> Result<Template, Redirect> {
    web::get_sesssion_from_cookies(&mut cookies)
        .map(|session| {
            let new_session = web::renew_session(&mut cookies, session);
            let mut context = Context::new();
            context.add("username", &new_session.get_user().username);
            let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
            let events = client.get_events().unwrap();
            context.add("events", &events);
            Template::render("events", &context)
        })
        .ok_or_else(|| Redirect::to("/login"))
}

#[get("/events?<sport_type>")]
pub fn get_events_query(
    mut cookies: Cookies,
    config: State<WebConfig>,
    sport_type: Option<SportType>,
) -> Result<Template, Redirect> {
    web::get_sesssion_from_cookies(&mut cookies)
        .map(|session| {
            let new_session = web::renew_session(&mut cookies, session);
            let mut context = Context::new();
            context.add("username", &new_session.get_user().username);

            let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
            let mut events = client.get_events().unwrap();

            if let Some(sport) = sport_type {
                events.retain(|ref event| event.sport == sport.sport_type);
            }

            context.add("events", &events);
            Template::render("events", &context)
        })
        .ok_or_else(|| Redirect::to("/login"))
}
