use client::Client;
use rocket::request::FlashMessage;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;
use session::Session;
use tera::Context;
use web::login;
use web::WebConfig;

#[derive(FromForm)]
pub struct SportType {
    pub sport_type: String,
}

#[get("/events/<event_id>")]
pub fn get_event(
    event_id: i32,
    config: State<WebConfig>,
    flash: Option<FlashMessage>,
    session: Session,
) -> Template {
    let mut context = Context::new();
    context.insert("username", &session.get_user().username);

    let client = Client::new(config.api_url.clone(), session.get_user().clone());
    let event = client.get_event(event_id).unwrap();
    if let Some(flash_message) = flash {
        context.insert("flash", flash_message.msg());
    }
    context.insert("event", &event);
    Template::render("event", &context)
}

#[get("/events/<_event_id>", rank = 2)]
pub fn get_event_redirect(_event_id: i32) -> Redirect {
    Redirect::to(uri!(login::login_page))
}

#[get("/events")]
pub fn get_events(config: State<WebConfig>, session: Session) -> Template {
    let mut context = Context::new();
    context.insert("username", &session.get_user().username);
    let client = Client::new(config.api_url.clone(), session.get_user().clone());
    let events = client.get_events().unwrap();
    context.insert("events", &events);
    Template::render("events", &context)
}

#[get("/events", rank = 2)]
pub fn get_events_redirect() -> Redirect {
    Redirect::to(uri!(login::login_page))
}

#[get("/events?<sport_type..>")]
pub fn get_events_query(
    config: State<WebConfig>,
    sport_type: Option<Form<SportType>>,
    session: Session,
) -> Template {
    let mut context = Context::new();
    context.insert("username", &session.get_user().username);

    let client = Client::new(config.api_url.clone(), session.get_user().clone());
    let mut events = client.get_events().unwrap();

    if let Some(sport) = sport_type {
        events.retain(|ref event| event.sport == sport.sport_type);
    }

    context.insert("events", &events);
    Template::render("events", &context)
}