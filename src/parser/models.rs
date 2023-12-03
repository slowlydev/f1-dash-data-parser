use std::collections::HashMap;

use serde::Deserialize;

use super::deserializer::{inflate_zlib, kf_remover, route_message_type};

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
    #[serde(deserialize_with = "route_message_type")]
    pub a: (String, DataType, String),
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
    pub tla_rcm: data::TlaRcm,

    #[serde(deserialize_with = "kf_remover")]
    pub driver_list: HashMap<String, data::DriverList>,

    #[serde(rename = "CarData.z", deserialize_with = "inflate_zlib")]
    pub car_data: data::CarData,
    #[serde(rename = "Position.z", deserialize_with = "inflate_zlib")]
    pub position: data::Positions,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DataType {
    Heartbeat(data::Heartbeat),
    ExtrapolatedClock(data::ExtrapolatedClock),
    TopThree(data::TopThree),
    TimingStats(data::TimingStats),
    TimingAppData(data::TimingAppData),
    WeatherData(data::WeatherData),
    TrackStatus(data::TrackStatus),
    RaceControlMessages(data::RaceControlMessages),
    SessionInfo(data::SessionInfo),
    SessionData(data::SessionData),
    LapCount(data::LapCount),
    TimingData(data::TimingData),
    TeamRadio(data::TeamRadio),

    TlaRcm(data::TlaRcm),

    #[serde(deserialize_with = "kf_remover")]
    DriverList(HashMap<String, data::DriverList>),

    #[serde(deserialize_with = "inflate_zlib")]
    CarData(data::CarData),
    #[serde(deserialize_with = "inflate_zlib")]
    Positions(data::Positions),
}
