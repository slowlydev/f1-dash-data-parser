use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use super::deserializer::{
    inflate_zlib, inflate_zlib_variant_car, inflate_zlib_variant_pos, kf_remover,
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
    pub timing_data: data::TimingData,
    pub team_radio: Option<data::TeamRadio>, // TODO remove after testing
    pub tla_rcm: Option<data::TlaRcm>,       // TODO remove after testing
    pub lap_count: Option<data::LapCount>,

    #[serde(deserialize_with = "kf_remover")]
    pub driver_list: HashMap<String, data::DriverList>,

    #[serde(rename = "CarData.z", deserialize_with = "inflate_zlib")]
    pub car_data: data::CarData,
    #[serde(rename = "Position.z", deserialize_with = "inflate_zlib")]
    pub position: data::Positions,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Update {
    Heartbeat(markers::HeartbeatMarker, updates::Heartbeat, String),
    TopThree(markers::TopThreeMarker, updates::TopThree, String),
    TimingStats(markers::TimingStatsMarker, updates::TimingStats, String),
    TimingAppData(markers::TimingAppDataMarker, updates::TimingAppData, String),
    WeatherData(markers::WeatherDataMarker, updates::WeatherData, String),
    TrackStatus(markers::TrackStatusMarker, updates::TrackStatus, String),
    SessionInfo(markers::SessionInfoMarker, updates::SessionInfo, String),
    LapCount(markers::LapCountMarker, updates::LapCount, String),
    TimingData(markers::TimingDataMarker, updates::TimingData, String),
    TeamRadio(markers::TeamRadioMarker, updates::TeamRadio, String),
    TlaRcm(markers::TlaRcmMarker, updates::TlaRcm, String),

    ExtrapolatedClock(
        markers::ExtrapolatedClockMarker,
        updates::ExtrapolatedClock,
        String,
    ),

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
    CarData(markers::CarDataMarker, updates::CarData, String),

    #[serde(deserialize_with = "inflate_zlib_variant_pos")]
    Positions(markers::PositionsMarker, updates::Positions, String),

    DriverList(markers::DriverListMarker, Value, String),
}
