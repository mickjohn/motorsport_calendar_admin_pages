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
    pub id: i32,
    pub name: String,
    pub time: DtWrapper,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DtWrapper(pub DateTime<FixedOffset>);

impl<'v> FromFormValue<'v> for DtWrapper {
    type Error = ();

    fn from_form_value(form_value: &'v RawStr) -> Result<DtWrapper, ()> {
        // println!("{:#?}", DateTime::parse_from_str("2018-08-24T10:00:00+09:30", "%Y-%m-$dT%H:%M:%S%z"));
        match DateTime::parse_from_str(form_value, "%+") {
            Ok(dt) => Ok(DtWrapper(dt)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, FromForm)]
pub struct NewSession {
    pub name: String,
    pub time: String,
}
