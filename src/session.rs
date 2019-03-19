use chrono::prelude::*;
use chrono::Duration;
use rand::distributions::*;
use rand::{thread_rng, Rng};
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FromRequest, Request};
use rocket::State;
use rocket::http::{Cookie, Cookies};
use std::collections::HashMap;
use std::iter;
use user::UserWithPlaintextPassword;
use web::SessionStoreArc;

const SESSION_ID_LEN: &usize = &64;
const SESSION_DURATION_SECS: &i64 = &(60 * 60); //60 minutes
const SESSION_RENEW_PERIOD: &i64 = &(30 * 60); // 10 minutes

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

    pub fn get_sesssion_from_cookies(
        cookies: &mut Cookies,
        session_store: &SessionStore,
    ) -> Option<Session> {
        cookies
            .get_private(SESSION_COOKIE_NAME)
            .and_then(|session_cookie| session_store.get(session_cookie.value()).cloned())
    }

    fn generate_session_id() -> String {
        let mut rng = thread_rng();
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(*SESSION_ID_LEN)
            .collect::<String>()
    }

    pub fn generate_cookie_string(&self) -> String {
        format!(
            "{}={}; HttpOnly; Secure; Expires={}; path=/",
            SESSION_COOKIE_NAME,
            self.get_id().to_string(),
            self.get_expires_date_string(),
        )
        .to_string()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = ();

    // fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
    //     match request.guard::<State<SessionStoreArc>>() {
    //         Success(session_store_arc) => {
    //             let mut session_store = session_store_arc.write().unwrap();
    //             let mut cookies = request.cookies();
    //             web::get_sesssion_from_cookies(&mut cookies, &session_store)
    //                 .map(|session| {
    //                     let renewed_session =
    //                         web::renew_session(&mut cookies, &mut session_store, session);
    //                     Success(renewed_session)
    //                 })
    //                 .unwrap_or_else(|| Forward(()))
    //         }
    //         Forward(_) => Forward(()),
    //         Failure(_) => Failure((Status::BadRequest, ())),
    //     }
    // }

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        match request.guard::<State<SessionStoreArc>>() {
            Success(session_store_arc) => {
                let mut session_store = session_store_arc.write().unwrap();
                let mut cookies = request.cookies();
                Session::get_sesssion_from_cookies(&mut cookies, &session_store)
                    .map(|session| {
                        if let Some(renewed_session) =
                            session_store.renew_if_close_to_expiry(&session)
                        {
                            let cookie_string = renewed_session.generate_cookie_string();
                            cookies.remove_private(Cookie::named(SESSION_COOKIE_NAME));
                            cookies.add_private(Cookie::parse(cookie_string).unwrap());
                            Success(renewed_session)
                        } else {
                            Success(session)
                        }
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
    renewal_period: i64,
}

impl SessionStore {
    pub fn new() -> Self {
        SessionStore {
            store: HashMap::new(),
            renewal_period: *SESSION_RENEW_PERIOD,
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

    pub fn renew_if_close_to_expiry(&mut self, old_session: &Session) -> Option<Session> {
        debug!(
            "Renewing session {} if it's within the renewal period",
            old_session.get_id()
        );
        let renewal_period = Duration::seconds(self.renewal_period);
        let now = Utc::now();
        if old_session.get_expires().signed_duration_since(now) <= renewal_period
            && old_session.get_expires().signed_duration_since(now) >= Duration::seconds(0)
        {
            Some(self.renew(old_session.clone()))
        } else {
            None
        }
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
