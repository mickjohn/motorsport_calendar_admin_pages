use chrono::prelude::*;
use chrono::Duration;
use rand::distributions::*;
use rand::{thread_rng, Rng};
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FromRequest, Request};
use rocket::State;
use std::collections::HashMap;
use std::iter;
use user::UserWithPlaintextPassword;
use web;
use web::SessionStoreArc;

const SESSION_ID_LEN: &usize = &64;
const SESSION_DURATION_SECS: &i64 = &(5 * 60);
//                                       Sun, 15 Jul 2012 00:00:01 GMT
const SESSION_COOKIE_DATE_FORMAT: &str = "%a, %d %h %Y %H:%M:%S GMT";
pub const SESSION_COOKIE_NAME: &str = "session_id";

#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    id: String,
    user: UserWithPlaintextPassword,
    expires: DateTime<Utc>,
}

impl Session {
    pub fn new_with_user(user: UserWithPlaintextPassword) -> Self {
        Session {
            id: Session::generate_session_id(),
            user,
            expires: Utc::now() + Duration::seconds(*SESSION_DURATION_SECS),
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_user(&self) -> &UserWithPlaintextPassword {
        &self.user
    }

    pub fn get_expires(&self) -> &DateTime<Utc> {
        &self.expires
    }

    pub fn get_expires_date_string(&self) -> String {
        self.expires.format(SESSION_COOKIE_DATE_FORMAT).to_string()
    }

    pub fn renew(self) -> Self {
        Session {
            id: Session::generate_session_id(),
            user: self.user,
            expires: Utc::now() + Duration::seconds(*SESSION_DURATION_SECS),
        }
    }

    fn generate_session_id() -> String {
        let mut rng = thread_rng();
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(*SESSION_ID_LEN)
            .collect::<String>()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        match request.guard::<State<SessionStoreArc>>() {
            Success(session_store_arc) => {
                let mut session_store = session_store_arc.write().unwrap();
                let mut cookies = request.cookies();
                web::get_sesssion_from_cookies(&mut cookies, &session_store)
                    .map(|session| {
                        let renewed_session =
                            web::renew_session(&mut cookies, &mut session_store, session);
                        Success(renewed_session)
                    })
                    .unwrap_or_else(|| Forward(()))
            }
            Forward(_) => Forward(()),
            Failure(_) => Failure((Status::BadRequest, ())),
        }
    }
}

#[derive(Debug)]
pub struct SessionStore {
    store: HashMap<String, Session>,
}

impl SessionStore {
    pub fn new() -> Self {
        SessionStore {
            store: HashMap::new(),
        }
    }

    pub fn add(&mut self, s: Session) {
        let k = s.get_id().to_string();
        self.store.insert(k, s);
    }

    pub fn remove(&mut self, sid: &str) {
        self.store.remove(sid);
    }

    pub fn get(&self, sid: &str) -> Option<&Session> {
        self.store.get(sid)
    }

    pub fn renew(&mut self, old_session: Session) -> Session {
        let old_session_id = old_session.get_id().to_string();
        let new_session = old_session.renew();
        let new_id = new_session.get_id().to_string();
        self.store.insert(new_id, new_session.clone());
        self.store.remove(&old_session_id);
        new_session
    }

    pub fn clean(&mut self) {
        let now = Utc::now();
        self.store.retain(|_, ref mut v| {
            if v.get_expires() >= &now {
                true
            } else {
                info!("Removing expired session {}", v.get_id());
                false
            }
        });
    }
}
