use postgres::{Connection, Result};

use std::str::from_utf8;

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionJSON {
    pub tabs: Tabs,
    pub system: System,
    #[serde(rename="GeoLocation")]
    pub geo_location: GeoLocation
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tabs {
    #[serde(rename="windowIndex")]
    pub window_index: i64,
    pub highlighted: bool,
    pub pinned: bool,
    pub audible: Option<bool>,
    pub url: String,
    pub title: String,
    pub incognito: bool,
    #[serde(rename="favIconUrl")]
    pub fav_icon_url: String
}

// Currently Ignoring cpu info sent by the extension
#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    pub memory: MemoryInfo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemoryInfo {
    #[serde(rename="availableCapacity")]
    pub available_capacity: i64,
    pub capacity: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub accuracy: f64,
    #[serde(rename="altitudeAccuracy")]
    pub altitude_accuracy: f64,
    pub heading: f64,
    pub speed: f64
}

pub struct Session {
    pub id: i64,
    pub account_id: i64,
    pub session_json: SessionJSON
}

pub fn insert_session(conn: &::PostgresPooledConnection, account_id: i64, session_json: SessionJSON) -> Result<u64, postgres::error::Error> {

    match conn.prepare("INSERT INTO sessions (account_id, session) VALUES ($1, $2)") {
        Ok(stmt) => stmt.execute(&[&account_id, &session_json]),
        Err(err) => Err(err),
    }
}
