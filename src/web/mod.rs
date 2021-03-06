use super::config;
use super::session::{Session, SessionStore};
use rocket;
use rocket::http::Cookies;
use rocket::Rocket;
use rocket_contrib::templates::Template;
use session;

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

mod create;
mod dashboard;
mod delete;
mod events;
mod login;
mod static_routes;
mod update;

pub type SessionStoreArc = Arc<RwLock<SessionStore>>;

#[derive(Debug)]
pub struct WebConfig {
    content_root: String,
    css_root: String,
    js_root: String,
    api_url: String,
    cookie_cleaner_interval_seconds: u64,
}

impl<'a> From<&'a config::Config> for WebConfig {
    fn from(c: &'a config::Config) -> Self {
        WebConfig {
            content_root: c.content_root.clone(),
            css_root: c.css_root.clone(),
            js_root: c.js_root.clone(),
            api_url: format!("{}:{}", c.api_server_host, c.api_server_port).to_string(),
            cookie_cleaner_interval_seconds: c.cookie_cleaner_interval_seconds,
        }
    }
}

fn init_rocket(web_config: WebConfig) -> Rocket {
    info!("Web config is as follows:\n{:#?}", web_config);
    // Use Arc to manage shared mutiple state.
    let session_store = Arc::new(RwLock::new(SessionStore::new()));
    let session_store_for_thread = Arc::clone(&session_store);

    info!("Starting up cookie cleaner thread");
    let interval = Duration::from_secs(web_config.cookie_cleaner_interval_seconds);
    thread::Builder::new()
        .name("cookie_cleaner".to_string())
        .spawn(move || loop {
            session_store_for_thread.write().unwrap().clean();
            thread::sleep(interval);
        })
        .unwrap();

    info!("Launching Rocket...");
    rocket::ignite()
        .mount(
            "/",
            routes![
                static_routes::static_css,
                static_routes::static_js,
                static_routes::index,
                static_routes::index_redirect,
                static_routes::internal_server_error,
                events::get_events,
                events::get_events_redirect,
                events::get_events_query,
                events::get_event,
                events::get_event_redirect,
                dashboard::dashboard,
                dashboard::dashboard_redirect,
                login::login_page_flash_message,
                login::login_page,
                login::login_user,
                login::logout_user,
                update::update_event,
                update::update_sessions,
                create::create_event,
                create::create_session,
                delete::delete_event,
                delete::delete_session,
            ],
        )
        .attach(Template::fairing())
        .manage(web_config)
        .manage(session_store)
}

pub fn start(web_config: WebConfig) {
    init_rocket(web_config).launch();
}

pub fn get_sesssion_from_cookies(
    cookies: &mut Cookies,
    session_store: &SessionStore,
) -> Option<Session> {
    cookies
        .get_private(session::SESSION_COOKIE_NAME)
        .and_then(|session_cookie| session_store.get(session_cookie.value()).cloned())
}