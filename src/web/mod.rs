use super::config;
// use super::env_log;
use super::session::Session;
use rocket;
use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket::response::NamedFile;
use rocket::{Rocket, State};
use rocket_contrib::Template;
use session;
use chrono::prelude::*;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::{thread, time};

mod dashboard;
mod login;

lazy_static! {
    static ref SESSION_MAP: RwLock<HashMap<String, Session>> = RwLock::new(HashMap::new());
}

pub struct WebConfig {
    content_root: String,
    css_root: String,
    js_root: String,
    api_url: String,
}

impl<'a> From<&'a config::Config> for WebConfig {
    fn from(c: &'a config::Config) -> Self {
        WebConfig {
            content_root: c.content_root.clone(),
            css_root: c.css_root.clone(),
            js_root: c.js_root.clone(),
            api_url: format!("{}:{}", c.api_server_host, c.api_server_port).to_string(),
        }
    }
}

#[get("/css/<file..>")]
fn static_css(file: PathBuf, config: State<WebConfig>) -> Option<NamedFile> {
    NamedFile::open(
        Path::new(&config.content_root)
            .join(&config.css_root)
            .join(file),
    ).ok()
}

#[get("/js/<file..>")]
fn static_js(file: PathBuf, config: State<WebConfig>) -> Option<NamedFile> {
    NamedFile::open(
        Path::new(&config.content_root)
            .join(&config.js_root)
            .join(file),
    ).ok()
}

#[get("/")]
fn index(cookies: Cookies) -> Result<Template, Redirect> {
    dashboard::dashboard(cookies)
}

fn clean_expired_cookies() {
    let now = Utc::now();
    SESSION_MAP.write().unwrap().retain(|_, ref mut v| v.get_expires() <= &now);
}

fn init_rocket(web_config: WebConfig) -> Rocket {
    // Logging framework not init at this point
    // that's why I'm using println
    println!("Starting cookie cleaner thread");
    thread::spawn(move || loop {
        clean_expired_cookies();
        thread::sleep(time::Duration::from_secs(5));
    });

    rocket::ignite()
        .mount(
            "/",
            routes![
                static_css,
                static_js,
                index,
                dashboard::dashboard,
                login::login_page_flash_message,
                login::login_page,
                login::login_user,
                login::logout_user,
            ],
        )
        .attach(Template::fairing())
        .manage(web_config)
}

pub fn start(web_config: WebConfig) {
    init_rocket(web_config).launch();
}

fn get_sesssion_from_cookies(cookies: &mut Cookies) -> Option<Session> {
    // Adding extra scope to limit the read lock
    let session = {
        let session_map = SESSION_MAP.read().unwrap();
        cookies
            .get_private(session::SESSION_COOKIE_NAME)
            .and_then(|session_cookie| session_map.get(session_cookie.value()).cloned())
    };
    session
}

/*
 * 1) Get old session ID
 * 2) Renew Session to get new Session
 * 3) Remove old Cookie
 * 4) Add new Cookie
 * 5) Remove Old session
 * 6) Add New Session
 * 7) Done
 */
fn renew_session(cookies: &mut Cookies, session: Session) -> Session {
    let new_session = {
        let sesion_map = SESSION_MAP.write().unwrap();
        let old_session_id = session.get_id().to_string();
        let new_session = session.renew();
        cookies.remove_private(Cookie::named(session::SESSION_COOKIE_NAME));

        let cookie_string = format!(
            "{}={}; HttpOnly; Secure; Expires={}; path=/",
            session::SESSION_COOKIE_NAME,
            new_session.get_id().to_string(),
            new_session.get_expires_date_string(),
            );

        cookies.add_private(Cookie::parse(cookie_string).unwrap());
        let key = new_session.get_id().to_string();
        info!("session created for user {}", new_session.get_user().username);
        SESSION_MAP.write().unwrap().insert(key, new_session.clone());
        new_session
    };
    new_session
}
