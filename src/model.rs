use chrono::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventUpdate {
    pub sport: String,
    pub round: i32,
    pub country: String,
    pub location: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionUpdate {
    pub id: i32,
    pub name: String,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewSession {
    pub id: i32,
    pub name: String,
    pub time: String,
}
