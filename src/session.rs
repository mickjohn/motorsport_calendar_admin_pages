use chrono::prelude::*;
use chrono::Duration;
use rand::distributions::*;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::iter;
use std::sync::RwLock;
use std::{thread, time};
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
}

#[derive(Debug)]
pub struct SessionStore {
    store: RwLock<HashMap<String, Session>>,
}

impl SessionStore {
    pub fn new() -> Self {
        SessionStore {
            store: RwLock::new(HashMap::new()),
        }
    }

    pub fn add(&mut self, s: Session) {
        let k = s.get_id().to_string();
        self.store.write().unwrap().insert(k, s);
    }

    pub fn remove(&mut self, sid: &str) {
        self.store.write().unwrap().remove(sid);
    }

    pub fn renew(&mut self, old_session: Session) -> Session {
        let old_session_id = old_session.get_id().to_string();
        let new_session = old_session.renew();
        let new_id = new_session.get_id().to_string();
        let mut store = self.store.write().unwrap();
        store.insert(new_id, new_session.clone());
        store.remove(&old_session_id);
        new_session
    }

    fn clean(&mut self) {
        let now = Utc::now();
        self.store
            .write()
            .unwrap()
            .retain(|_, ref mut v| v.get_expires() >= &now);
    }
}
