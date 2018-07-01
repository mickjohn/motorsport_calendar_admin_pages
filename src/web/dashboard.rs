// use super::log;
use rocket::http::Cookies;
use rocket::response::Redirect;
use rocket_contrib::Template;
use std::collections::HashMap;
use web;

#[get("/dashboard")]
pub fn dashboard(mut cookies: Cookies) -> Result<Template, Redirect> {
    // web::get_sesssion_from_cookies_and_renew(&mut cookies)
    web::get_sesssion_from_cookies(&mut cookies)
        .map(|session| {
            debug!("Found session cookie!");
            debug!("Renewing session cookie!");
            let new_session = web::renew_session(&mut cookies, session);
            let mut context = HashMap::new();
            context.insert(
                "username".to_string(),
                // session.get_user().username.to_string(),
                new_session.get_user().username.to_string(),
            );
            Template::render("dashboard", &context)
        })
        .ok_or_else(|| Redirect::to("/login"))
}
