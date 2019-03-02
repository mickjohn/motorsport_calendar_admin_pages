use chrono::*;
use rocket::http::RawStr;
use rocket::request::{FormItem, FormItems, FromForm, FromFormValue};

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

#[derive(Debug, Clone)]
pub struct SessionsUpdate {
    pub sessions: Vec<(i32, SessionUpdate)>,
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

fn get_id_from_form_item(item: &FormItem, key_name: &str) -> Option<i32> {
    if item.key == key_name {
        let id_string = item.value.url_decode().unwrap();
        if let Ok(id) = id_string.parse::<i32>() {
            Some(id)
        } else {
            None
        }
    } else {
        None
    }
}

fn get_name_from_form_item(item: &FormItem, key_name: &str) -> Option<String> {
    if item.key == key_name {
        Some(item.value.url_decode().unwrap().to_string())
    } else {
        None
    }
}

fn get_time_from_form_item(item: &FormItem, key_name: &str) -> Option<DtWrapper> {
    if item.key == key_name {
        let time_string = &item.value.url_decode().unwrap();
        if let Ok(dt) = NaiveDateTime::parse_from_str(&time_string, "%Y-%m-%dT%H:%M:%S") {
            Some(DtWrapper(dt))
        } else {
            None
        }
    } else {
        None
    }
}

// Hacky way of submitting a list of session updates in one form
impl<'f> FromForm<'f> for SessionsUpdate {
    type Error = ();

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<SessionsUpdate, ()> {
        let iter = items.into_iter();
        let mut sessions = Vec::new();
        let mut index = 0;

        // SessionUpdate has 5 fields. So take 5 fields at a time. Don't care about the last two.
        loop {
            let id_key = format!("id_{}", index);
            let name_key = format!("name_{}", index);
            let time_key = format!("time_{}", index);

            if let (Some(id_form_item), Some(name_form_item), Some(time_form_item), _, _) =
                (iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
            {
                let id = get_id_from_form_item(&id_form_item, &id_key);
                let name = get_name_from_form_item(&name_form_item, &name_key);
                let time = get_time_from_form_item(&time_form_item, &time_key);

                if let (Some(id), Some(name), Some(time)) = (id, name, time) {
                    sessions.push((id, SessionUpdate { name, time }));
                }
            } else {
                break;
            }
            index += 1;
        }
        Ok(SessionsUpdate { sessions })
    }
}
