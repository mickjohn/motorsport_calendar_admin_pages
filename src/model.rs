use chrono::*;
use rocket::request::FromFormValue;
use rocket::http::RawStr;

#[derive(Debug, Clone, Deserialize, Serialize, FromForm)]
pub struct EventUpdate {
    pub sport: String,
    pub round: i32,
    pub country: String,
    pub location: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromForm)]
pub struct SessionUpdate {
    pub id: i32,
    pub name: String,
    // pub time: DateTime<Utc>,
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

        // match form_value.parse::<usize>() {
        //     Ok(age) if age >= 21 => Ok(AdultAge(age)),
        //     _ => Err(()),
        // }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewSession {
    pub id: i32,
    pub name: String,
    pub time: String,
}
