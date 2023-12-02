use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use super::deserializer::{inflate_zlib, kf_remover};

pub mod data;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct SocketMessage {
    pub m: Option<Vec<Update>>,
    pub r: Option<Data>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct Update {
    pub a: (String, Value, String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub heartbeat: data::Heartbeat,
    pub extrapolated_clock: data::ExtrapolatedClock,
    pub top_three: data::TopThree,
    pub timing_stats: data::TimingStats,
    pub timing_app_data: data::TimingAppData,
    pub weather_data: data::WeatherData,
    pub track_status: data::TrackStatus,
    pub race_control_messages: data::RaceControlMessages,
    pub session_info: data::SessionInfo,
    pub session_data: data::SessionData,
    pub lap_count: data::LapCount,
    pub timing_data: data::TimingData,
    pub team_radio: data::TeamRadio,

    #[serde(deserialize_with = "kf_remover")]
    pub driver_list: HashMap<String, data::DriverList>,

    #[serde(rename = "CarData.z", deserialize_with = "inflate_zlib")]
    pub car_data: data::CarData,
    #[serde(rename = "Position.z", deserialize_with = "inflate_zlib")]
    pub position: data::Positions,
}
