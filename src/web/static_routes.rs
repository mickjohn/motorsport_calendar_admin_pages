use super::dashboard;
use super::WebConfig;
use session::Session;

use rocket::request::FlashMessage;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::templates::Template;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[get("/css/<file..>")]
pub fn static_css(file: PathBuf, config: State<WebConfig>) -> Option<NamedFile> {
    NamedFile::open(
        Path::new(&config.content_root)
            .join(&config.css_root)
            .join(file),
    )
    .ok()
}

#[get("/js/<file..>")]
pub fn static_js(file: PathBuf, config: State<WebConfig>) -> Option<NamedFile> {
    NamedFile::open(
        Path::new(&config.content_root)
            .join(&config.js_root)
            .join(file),
    )
    .ok()
}

#[get("/")]
pub fn index(config: State<WebConfig>, session: Session) -> Template {
    dashboard::dashboard(config, session)
}

#[get("/500_error.html")]
pub fn internal_server_error(flash: Option<FlashMessage>) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    if let Some(ref flash_message) = flash {
        context.insert("flash", flash_message.msg());
    }
    Template::render("500_error", &context)
}
