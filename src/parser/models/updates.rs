use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

use crate::parser::deserializer::{
    flatten_map_to_vec, flatten_map_to_vec_optional, kf_remover, map_or_vec_to_vec_optional,
    parse_chrono_date,
};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Driver {
    pub racing_number: String,
    pub broadcast_name: String,
    pub full_name: String,
    pub tla: String,
    pub line: i64,
    pub team_name: String,
    pub team_colour: String,
    pub first_name: String,
    pub last_name: String,
    pub reference: String,
    pub headshot_url: Option<String>,
    pub country_code: String,
    pub name_format: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RaceControlMessages {
    #[serde(deserialize_with = "flatten_map_to_vec")]
    pub messages: Vec<RaceControlMessage>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RaceControlMessage {
    #[serde(deserialize_with = "parse_chrono_date")]
    pub utc: DateTime<Utc>,
    pub lap: Option<i64>,
    pub category: Category,
    pub flag: Option<Flag>,
    pub scope: Option<Scope>,
    pub message: String,
    pub sector: Option<i64>,
    pub status: Option<String>,
    pub mode: Option<String>,
    pub racing_number: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Category {
    Drs,
    Flag,
    Other,
    SafetyCar,
    CarEvent,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Flag {
    #[serde(rename = "CHEQUERED")]
    Chequered,
    #[serde(rename = "CLEAR")]
    Clear,
    #[serde(rename = "DOUBLE YELLOW")]
    DoubleYellow,
    #[serde(rename = "GREEN")]
    Green,
    #[serde(rename = "YELLOW")]
    Yellow,
    #[serde(rename = "RED")]
    Red,
    #[serde(rename = "BLUE")]
    Blue,
    #[serde(rename = "BLACK AND WHITE")]
    BlackAndWhite,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Scope {
    Sector,
    Track,
    Driver,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SessionData {
    // #[serde(deserialize_with = "flatten_map_to_vec_optional")]
    pub series: Option<HashMap<String, Lap>>,
    // #[serde(deserialize_with = "flatten_map_to_vec_optional")]
    pub status_series: Option<HashMap<String, Status>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Lap {
    #[serde(deserialize_with = "parse_chrono_date")]
    pub utc: DateTime<Utc>,
    pub lap: Option<i64>,
    pub qualifying_part: Option<i8>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Status {
    #[serde(deserialize_with = "parse_chrono_date")]
    pub utc: DateTime<Utc>,
    pub track_status: Option<String>,
    pub session_status: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TeamRadio {
    #[serde(deserialize_with = "flatten_map_to_vec")]
    pub captures: Vec<super::Capture>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TimingAppData {
    #[serde(deserialize_with = "kf_remover")]
    pub lines: HashMap<String, TimingAppDataLine>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TimingAppDataLine {
    pub racing_number: Option<String>,
    pub line: Option<i64>,
    pub grid_pos: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "map_or_vec_to_vec_optional")]
    pub stints: Option<Vec<Stint>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Stint {
    pub lap_time: Option<String>,
    pub lap_number: Option<i64>,
    pub lap_flags: Option<i64>,
    pub compound: Option<Compound>,
    pub new: Option<String>,
    pub tyres_not_changed: Option<String>,
    pub total_laps: Option<i64>,
    pub start_laps: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Compound {
    #[serde(rename = "HARD")]
    Hard,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "SOFT")]
    Soft,
    #[serde(rename = "INTERMEDIATE")]
    Intermediate,
    #[serde(rename = "WET")]
    Wet,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TimingData {
    pub session_part: Option<i8>,
    #[serde(deserialize_with = "kf_remover")]
    pub lines: HashMap<String, TimingDataLine>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TimingDataLine {
    pub gap_to_leader: Option<String>,
    pub interval_to_position_ahead: Option<IntervalToPositionAhead>,
    pub line: Option<i64>,
    pub position: Option<String>,
    pub show_position: Option<bool>,
    pub racing_number: Option<String>,
    pub retired: Option<bool>,
    pub in_pit: Option<bool>,
    pub pit_out: Option<bool>,
    pub knocked_out: Option<bool>,
    pub stopped: Option<bool>,
    pub status: Option<i64>,
    pub number_of_laps: Option<i64>,
    pub number_of_pit_stops: Option<i64>,
    #[serde(default)]
    #[serde(deserialize_with = "flatten_map_to_vec_optional")]
    pub sectors: Option<Vec<Sector>>,
    pub speeds: Option<Speeds>,

    #[serde(default)]
    #[serde(deserialize_with = "flatten_map_to_vec_optional")]
    pub best_lap_times: Option<Vec<BestLapTime>>,

    pub best_lap_time: Option<BestLapTime>,
    pub last_lap_time: Option<LastLapTime>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BestLapTime {
    pub value: String,
    pub lap: Option<i64>,
    #[serde(rename = "_deleted")]
    pub deleted: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IntervalToPositionAhead {
    pub value: Option<String>,
    pub catching: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LastLapTime {
    pub value: Option<String>,
    pub status: Option<i64>,
    pub overall_fastest: Option<bool>,
    pub personal_fastest: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Sector {
    pub stopped: Option<bool>,
    pub previous_value: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "flatten_map_to_vec_optional")]
    pub segments: Option<Vec<Segment>>,
    pub value: Option<String>,
    pub status: Option<i64>,
    pub overall_fastest: Option<bool>,
    pub personal_fastest: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Segment {
    pub status: i64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Speeds {
    pub i1: Option<LastLapTime>,
    pub i2: Option<LastLapTime>,
    pub fl: Option<LastLapTime>,
    pub st: Option<LastLapTime>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TimingStats {
    #[serde(deserialize_with = "kf_remover")]
    pub lines: HashMap<String, TimingStatsLine>,
    pub session_type: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TimingStatsLine {
    pub line: Option<i64>,
    pub racing_number: Option<String>,
    pub personal_best_lap_time: Option<PersonalBestLapTime>,
    // pub best_sectors: Option<Vec<BestSector>>,
    // pub best_speeds: Option<BestSpeeds>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BestSector {
    pub value: String,
    pub position: Option<i64>,
    pub overall_fastest: Option<bool>,
    pub personal_fastest: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BestSpeeds {
    pub i1: Option<BestSector>,
    pub i2: Option<BestSector>,
    pub fl: Option<BestSector>,
    pub st: Option<BestSector>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PersonalBestLapTime {
    pub lap: Option<i64>,
    pub position: Option<i64>,
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TopThree {
    #[serde(deserialize_with = "flatten_map_to_vec")]
    pub lines: Vec<LineElement>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LineElement {
    pub position: Option<String>,
    pub show_position: Option<bool>,
    pub racing_number: Option<String>,
    pub tla: Option<String>,
    pub broadcast_name: Option<String>,
    pub full_name: Option<String>,
    pub team: Option<String>,
    pub team_colour: Option<String>,
    pub lap_time: Option<String>,
    pub lap_state: Option<i64>,
    pub diff_to_ahead: Option<String>,
    pub diff_to_leader: Option<String>,
    pub overall_fastest: Option<bool>,
    pub personal_fastest: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PitLaneTimeCollection {
    pub pit_times: HashMap<String, PitTimeEnum>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum PitTimeEnum {
    PitTime(PitTime),
    Deleted(Vec<String>),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PitTime {
    pub racing_number: String,
    pub duration: String,
    pub lap: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LapSeries {
    pub lap_position: HashMap<String, String>,
}
