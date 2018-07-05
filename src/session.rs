use chrono::prelude::*;
use chrono::Duration;
use rand::distributions::*;
use rand::{thread_rng, Rng};
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
}

// #[derive(Debug)]
// pub struct LocalSessionDb {
//     session_map: HashMap<String, Session>,
// }

// impl LocalSessionDb {
//     pub fn new() -> LocalSessionDb {
//         LocalSessionDb {
//             session_map: HashMap::new(),
//         }
//     }

//     pub fn add_session(&mut self, session: Session) {
//         let key = session.id.clone();
//         self.session_map.insert(key, session);
//     }

//     pub fn remove_session(&mut self, session_id: String) {
//         self.session_map.remove(&session_id);
//     }

//     pub fn get_session(self, session_id: String) -> Option<Session> {
//         match self.session_map.get(&session_id) {
//             None => None,
//             Some(s) => Some(s.clone()),
//         }
//     }
// }
