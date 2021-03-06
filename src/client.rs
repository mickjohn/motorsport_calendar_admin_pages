use chrono::NaiveDateTime;
use model::*;
use motorsport_calendar_common::event::Event;
use reqwest::header;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client as ReqwestClient;
use reqwest::ClientBuilder;
use reqwest::Error as ReqwestError;
use reqwest::StatusCode;
use serde_json;
use serde_json::Error as SerdeError;

use super::user::UserWithPlaintextPassword as Uwpp;

#[derive(Debug, Serialize)]
struct ApiSessionUpdate {
    pub name: String,
    pub time: NaiveDateTime,
}

impl From<SessionUpdate> for ApiSessionUpdate {
    fn from(session_update: SessionUpdate) -> Self {
        ApiSessionUpdate {
            name: session_update.name,
            time: session_update.time.0,
        }
    }
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "You need to authenticate with Basic auth to access this resource.")]
    // LoginFail,
    // #[fail(display = "Request error")]
    ReqwestError(ReqwestError),

    #[fail(display = "Serde json error")]
    SerdeError(SerdeError),

    #[fail(display = "Failed to authenticate")]
    AuthError,
}

impl From<ReqwestError> for Error {
    fn from(request_error: ReqwestError) -> Self {
        Error::ReqwestError(request_error)
    }
}

impl From<SerdeError> for Error {
    fn from(serde_error: SerdeError) -> Self {
        Error::SerdeError(serde_error)
    }
}

pub struct Client {
    api_url: String,
    user: Uwpp,
}

impl Client {
    pub fn new(api_url: String, user: Uwpp) -> Self {
        Client { api_url, user }
    }

    fn http_client(&self) -> Result<ReqwestClient, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("json"));
        let client = ClientBuilder::new().default_headers(headers).build()?;
        Ok(client)
    }

    fn http_client_with_auth(&self) -> Result<ReqwestClient, Error> {
        let client = ClientBuilder::new().build()?;
        Ok(client)
    }

    fn json_http_client_with_auth(&self) -> Result<ReqwestClient, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("json"));
        let client = ClientBuilder::new().default_headers(headers).build()?;
        Ok(client)
    }

    pub fn get_events(&self) -> Result<Vec<Event>, Error> {
        let client = self.http_client()?;
        let url = format!("{}/events", self.api_url);
        let mut response = client.get(&url).send()?;
        let text = response.text()?;
        let events: Vec<Event> = serde_json::from_str(&text)?;
        Ok(events)
    }

    pub fn get_event(&self, id: i32) -> Result<Event, Error> {
        let client = self.http_client()?;
        let url = format!("{}/events/{}", self.api_url, id);
        let mut response = client.get(&url).send()?;
        let text = response.text()?;
        let event: Event = serde_json::from_str(&text)?;
        Ok(event)
    }

     pub fn authenticate(&self) -> Result<(), Error> {
        let client = self.http_client_with_auth()?;
        let url = format!("{}/authenticate", self.api_url);
        let response = client
            .post(&url)
            .basic_auth(self.user.username.clone(), Some(self.user.password.clone()))
            .send()?;
        match response.status() {
            StatusCode::OK => Ok(()),
            _ => Err(Error::AuthError),
        }
    }

    pub fn update_event(&self, updated_event: &EventUpdate, event_id: i32) -> Result<(), Error> {
        let client = self.json_http_client_with_auth()?;
        let body_string = serde_json::to_string(&updated_event).unwrap();
        let url = format!("{}/events/{}", self.api_url, event_id);
        client
            .put(&url)
            .basic_auth(
                &self.user.username.clone(),
                Some(self.user.password.clone()),
            )
            .body(body_string)
            .send()?;
        Ok(())
    }

    pub fn update_session(
        &self,
        updated_session: SessionUpdate,
        session_id: i32,
        event_id: i32,
    ) -> Result<(), Error> {
        let client = self.json_http_client_with_auth()?;
        let body_string = serde_json::to_string(&ApiSessionUpdate::from(updated_session)).unwrap();
        let url = format!(
            "{url}/events/{event_id}/sessions/{session_id}",
            url = self.api_url,
            event_id = event_id,
            session_id = session_id,
        );
        client
            .put(&url)
            .basic_auth(
                &self.user.username.clone(),
                Some(self.user.password.clone()),
            )
            .body(body_string)
            .send()?;
        Ok(())
    }

    pub fn create_session(&self, new_session: &NewSession, event_id: i32) -> Result<(), Error> {
        let client = self.json_http_client_with_auth()?;
        let body_string = serde_json::to_string(&new_session).unwrap();
        let url = format!(
            "{url}/events/{event_id}/create_session",
            url = self.api_url,
            event_id = event_id,
        );
        client
            .post(&url)
            .basic_auth(
                &self.user.username.clone(),
                Some(self.user.password.clone()),
            )
            .body(body_string)
            .send()?;
        Ok(())
    }

    pub fn create_event(&self, new_event: &NewEvent) -> Result<(), Error> {
        let client = self.json_http_client_with_auth()?;
        let body_string = serde_json::to_string(&new_event).unwrap();
        let url = format!("{}/events/create_event", self.api_url);
        client
            .post(&url)
            .basic_auth(
                &self.user.username.clone(),
                Some(self.user.password.clone()),
            )
            .body(body_string)
            .send()?;
        Ok(())
    }

    pub fn delete_session(&self, session_id: i32, event_id: i32) -> Result<(), Error> {
        let client = self.json_http_client_with_auth()?;
        let url = format!(
            "{url}/events/{event_id}/sessions/{session_id}",
            url = self.api_url,
            session_id = session_id,
            event_id = event_id,
        );
        client
            .delete(&url)
            .basic_auth(
                &self.user.username.clone(),
                Some(self.user.password.clone()),
            )
            .send()?;
        Ok(())
    }

    pub fn delete_event(&self, event_id: i32) -> Result<(), Error> {
        let client = self.json_http_client_with_auth()?;
        let url = format!("{}/events/{}", self.api_url, event_id);
        client
            .delete(&url)
            .basic_auth(
                &self.user.username.clone(),
                Some(self.user.password.clone()),
            )
            .send()?;
        Ok(())
    }
}
