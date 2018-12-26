use chrono::*;
use rocket::http::RawStr;
use rocket::request::FromFormValue;

#[derive(Debug, Clone, Deserialize, Serialize, FromForm)]
pub struct EventUpdate {
    pub sport: String,
    pub title: String,
    pub country: String,
    pub location: String,
    pub track: String,
}

pub type NewEvent = EventUpdate;

#[derive(Debug, Clone, Deserialize, Serialize, FromForm)]
pub struct SessionUpdate {
    pub name: String,
    pub time: DtWrapper,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DtWrapper(pub NaiveDateTime);

impl<'v> FromFormValue<'v> for DtWrapper {
    type Error = ();

    fn from_form_value(form_value: &'v RawStr) -> Result<DtWrapper, ()> {
        if let Ok(value) = form_value.percent_decode() {
            if let Ok(dt) = NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M:%S") {
                return Ok(DtWrapper(dt));
            }
        }
        Err(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, FromForm)]
pub struct NewSession {
    pub name: String,
    pub time: String,
}
