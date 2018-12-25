use client::Client;
use rocket::http::Cookies;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;
use tera::Context;
use web;
use web::WebConfig;

#[get("/events/<event_id>/sessions/<session_id>")]
pub fn get_session(
    mut cookies: Cookies,
    config: State<WebConfig>,
    event_id: i32,
    session_id: i32,
) -> Result<Template, Redirect> {
    web::get_sesssion_from_cookies(&mut cookies)
        .map(|session| {
            let new_session = web::renew_session(&mut cookies, session);
            let mut context = Context::new();
            context.insert("username", &new_session.get_user().username);

            let client = Client::new(config.api_url.clone(), new_session.get_user().clone());
            let event = client.get_event(event_id).unwrap();
            let session = client.get_session(event_id, session_id).unwrap();
            context.insert("event", &session);
            context.insert("session", &session);
            Template::render("session", &context)
        })
        .ok_or_else(|| Redirect::to(uri!(login::login_page)))
}
