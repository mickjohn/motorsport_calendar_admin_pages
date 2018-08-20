use chrono::prelude::*;
use chrono::Duration;
use rand::distributions::*;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::iter;
use user::UserWithPlaintextPassword;

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

    // pub fn create_session_cookie(&self) -> Cookie {
    //     let expiry = Tm {
    //         tm_sec: self.expires.second() as i32,
    //         tm_min: self.expires.minute() as i32,
    //         tm_hour: self.expires.hour() as i32,
    //         tm_mday: self.expires.day() as i32,
    //         tm_mon: self.expires.month0() as i32,
    //         tm_year: self.expires.year() as i32,
    //         tm_wday: self.expires.weekday().num_days_from_monday() as i32,
    //         tm_yday: self.expires.ordinal() as i32,
    //         tm_isdst: -1,
    //         tm_utcoff: 0,
    //         tm_nsec: 0,
    //     };

    //     Cookie::build(SESSION_COOKIE_NAME, "123")
    //         .http_only(true)
    //         .secure(true)
    //         .expires(cxpiry)
    //         .path("/")
    //         .finish()
    // }
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
