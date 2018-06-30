use super::super::user::UserWithPlaintextPassword;
use super::super::client::Client;
use super::WebConfig;
use rocket_contrib::Template;
use rocket::http::{Cookie, Cookies};
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket::State;
use session::{self, Session};
use std::collections::HashMap;
use web;

// Render page with flash message, e.g. 'incorrect username or pass',
// or 'you must be logged in to view this page'
#[get("/login")]
pub fn login_page_flash_message(flash: Option<FlashMessage>) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    if let Some(ref flash_message) = flash {
        context.insert("flash", flash_message.msg());
    }
    Template::render("index", &context)
}

#[get("/login", rank = 2)]
pub fn login_page(
    mut cookies: Cookies,
) -> Result<Template, Redirect> {
    web::get_sesssion_from_cookies(&mut cookies)
        .map(|session| {
            debug!("Found session cookie!");
            let mut context = HashMap::new();
            context.insert(
                "username".to_string(),
                session.get_user().username.to_string(),
            );
            Err(Redirect::to("/dashboard"))
        })
        .unwrap_or_else(|| {
            let context: HashMap<&str, &str> = HashMap::new();
            Ok(Template::render("index", &context))
        })
}

#[post("/login", data = "<user_data>")]
fn login_user(
    mut cookies: Cookies,
    user_data: Form<UserWithPlaintextPassword>,
    config: State<WebConfig>,
) -> Result<Redirect, Flash<Redirect>> {
    let user = user_data.into_inner();
    let login_result = Client::new(config.api_url.clone(), user.clone()).authenticate();
    match login_result {
        Ok(()) => {
            info!("{} validated, creating session...", user.username);
            create_session(user, &mut cookies);
            Ok(Redirect::to("/dashboard"))
        }
        Err(_) => {
            info!("Username {} failed to login", user.username);
            Err(Flash::error(Redirect::to("/"), "Invalid username or password"))
        }
    }
}

#[get("/logout")]
fn logout_user(mut cookies: Cookies) -> Flash<Redirect> {
    web::get_sesssion_from_cookies(&mut cookies)
        .map(|session| {
            debug!("Found session cookie, loging out user");
            let mut session_map = web::SESSION_MAP.write().unwrap();
            session_map.remove(&session.get_id().to_string());
            cookies.remove_private(Cookie::named(session::SESSION_COOKIE_NAME));
        });
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

fn create_session(user: UserWithPlaintextPassword, cookies: &mut Cookies) {
    let session = Session::new_with_user(user);
    let cookie_string = format!(
        "{}={}; HttpOnly; Secure; Expires={}; path=/",
        session::SESSION_COOKIE_NAME,
        session.get_id().to_string(),
        session.get_expires_date_string(),
    );
    cookies.add_private(Cookie::parse(cookie_string).unwrap());
    let k = session.get_id().to_string();
    info!("session created for user {}", session.get_user().username);
    web::SESSION_MAP.write().unwrap().insert(k, session);
}
