use std::collections::HashMap;

use chrono::{format::parse, DateTime, NaiveDateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

use super::deserializer::{
    inflate_zlib, inflate_zlib_variant_car, inflate_zlib_variant_pos, kf_remover, parse_chrono_date,
};

pub mod data;
pub mod markers;
pub mod updates;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct SocketMessage {
    pub m: Option<Vec<Message>>,
    pub r: Option<Data>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Message {
    pub a: Update,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub heartbeat: Heartbeat,
    pub extrapolated_clock: ExtrapolatedClock,
    pub top_three: data::TopThree,
    pub timing_stats: data::TimingStats,
    pub timing_app_data: data::TimingAppData,
    pub weather_data: WeatherData,
    pub track_status: TrackStatus,
    pub race_control_messages: data::RaceControlMessages,
    pub session_info: SessionInfo,
    pub session_data: data::SessionData,
    pub timing_data: data::TimingData,
    pub team_radio: Option<data::TeamRadio>,
    pub tla_rcm: Option<TlaRcm>,
    pub lap_count: Option<LapCount>,

    #[serde(deserialize_with = "kf_remover")]
    pub lap_series: HashMap<String, data::LapSeries>,

    #[serde(deserialize_with = "kf_remover")]
    pub driver_list: HashMap<String, data::DriverList>,

    #[serde(rename = "CarData.z", deserialize_with = "inflate_zlib")]
    pub car_data: CarData,
    #[serde(rename = "Position.z", deserialize_with = "inflate_zlib")]
    pub position: Positions,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Update {
    Heartbeat(markers::HeartbeatMarker, Heartbeat, String),
    TopThree(markers::TopThreeMarker, updates::TopThree, String),
    TimingStats(markers::TimingStatsMarker, updates::TimingStats, String),
    TimingAppData(markers::TimingAppDataMarker, updates::TimingAppData, String),
    WeatherData(markers::WeatherDataMarker, WeatherData, String),
    TrackStatus(markers::TrackStatusMarker, TrackStatus, String),
    SessionInfo(markers::SessionInfoMarker, SessionInfo, String),
    LapCount(markers::LapCountMarker, LapCount, String),
    TimingData(markers::TimingDataMarker, updates::TimingData, String),
    TeamRadio(markers::TeamRadioMarker, updates::TeamRadio, String),
    TlaRcm(markers::TlaRcmMarker, TlaRcm, String),

    ExtrapolatedClock(markers::ExtrapolatedClockMarker, ExtrapolatedClock, String),

    RaceControlMessages(
        markers::RaceControlMessagesMarker,
        updates::RaceControlMessages,
        String,
    ),

    PitLaneTimeCollection(
        markers::PitLaneTimeCollectionMarker,
        updates::PitLaneTimeCollection,
        String,
    ),

    LapSeries(
        markers::LapSeriesMarker,
        HashMap<String, updates::LapSeries>,
        String,
    ),

    SessionData(markers::SessionDataMarker, updates::SessionData, String),

    #[serde(deserialize_with = "inflate_zlib_variant_car")]
    CarData(markers::CarDataMarker, CarData, String),

    #[serde(deserialize_with = "inflate_zlib_variant_pos")]
    Positions(markers::PositionsMarker, Positions, String),

    DriverList(markers::DriverListMarker, Value, String),
}

const FORMAT1: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

impl Update {
    pub fn get_timestamp(&self) -> DateTime<Utc> {
        let timestamp = match &self {
            Update::Heartbeat(_, _, t) => t,
            Update::TopThree(_, _, t) => t,
            Update::TimingStats(_, _, t) => t,
            Update::TimingAppData(_, _, t) => t,
            Update::WeatherData(_, _, t) => t,
            Update::TrackStatus(_, _, t) => t,
            Update::SessionInfo(_, _, t) => t,
            Update::LapCount(_, _, t) => t,
            Update::TimingData(_, _, t) => t,
            Update::TeamRadio(_, _, t) => t,
            Update::TlaRcm(_, _, t) => t,
            Update::ExtrapolatedClock(_, _, t) => t,
            Update::RaceControlMessages(_, _, t) => t,
            Update::PitLaneTimeCollection(_, _, t) => t,
            Update::LapSeries(_, _, t) => t,
            Update::SessionData(_, _, t) => t,
            Update::CarData(_, _, t) => t,
            Update::Positions(_, _, t) => t,
            Update::DriverList(_, _, t) => t,
        };

        let dt = NaiveDateTime::parse_from_str(&timestamp, FORMAT1).unwrap();
        DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Heartbeat {
    #[serde(deserialize_with = "parse_chrono_date")]
    pub utc: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WeatherData {
    pub air_temp: String,
    pub humidity: String,
    pub pressure: String,
    pub rainfall: String,
    pub track_temp: String,
    pub wind_direction: String,
    pub wind_speed: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TlaRcm {
    #[serde(deserialize_with = "parse_chrono_date")]
    timestamp: DateTime<Utc>,
    message: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LapCount {
    pub current_lap: Option<i64>,
    pub total_laps: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TrackStatus {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SessionInfo {
    pub meeting: Meeting,
    pub archive_status: ArchiveStatus,
    pub key: i64,
    #[serde(rename = "Type")]
    pub session_info_type: String,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub gmt_offset: String,
    pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ArchiveStatus {
    pub status: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Meeting {
    pub key: i64,
    pub name: String,
    pub official_name: String,
    pub location: String,
    pub country: Country,
    pub circuit: Circuit,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Circuit {
    pub key: i64,
    pub short_name: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Country {
    pub key: i64,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Positions {
    pub position: Vec<PositionElement>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PositionElement {
    #[serde(deserialize_with = "parse_chrono_date")]
    pub timestamp: DateTime<Utc>,
    #[serde(deserialize_with = "kf_remover")]
    pub entries: HashMap<String, EntryValue>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EntryValue {
    pub status: DriverStatus,
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub enum DriverStatus {
    #[serde(rename = "OnTrack")]
    OnTrack,
    #[serde(rename = "OffTrack")]
    OffTrack,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ExtrapolatedClock {
    #[serde(deserialize_with = "parse_chrono_date")]
    pub utc: DateTime<Utc>,
    pub remaining: String,
    pub extrapolating: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CarData {
    pub entries: Vec<EntryElement>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EntryElement {
    #[serde(deserialize_with = "parse_chrono_date")]
    pub utc: DateTime<Utc>,
    #[serde(deserialize_with = "kf_remover")]
    pub cars: HashMap<String, Car>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Car {
    #[serde(deserialize_with = "kf_remover")]
    pub channels: HashMap<String, i64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Capture {
    #[serde(deserialize_with = "parse_chrono_date")]
    pub utc: DateTime<Utc>,
    pub racing_number: String,
    pub path: String,
}
