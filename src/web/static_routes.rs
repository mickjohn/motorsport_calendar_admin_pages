use super::dashboard;
use super::WebConfig;

use rocket::http::{Cookie, Cookies};
use rocket::request::FlashMessage;
use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::Template;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
fn index(cookies: Cookies, config: State<WebConfig>) -> Result<Template, Redirect> {
    dashboard::dashboard(cookies, config)
}

#[get("/500_error.html")]
fn internal_server_error(flash: Option<FlashMessage>) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    if let Some(ref flash_message) = flash {
        context.insert("flash", flash_message.msg());
    }
    Template::render("500_error", &context)
}
