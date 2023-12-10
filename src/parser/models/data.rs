use serde::Deserialize;
use std::collections::HashMap;

use crate::parser::deserializer::kf_remover;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CarData {
    pub entries: Vec<EntryElement>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EntryElement {
    pub utc: String,
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
pub struct DriverList {
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
pub struct ExtrapolatedClock {
    pub utc: String,
    pub remaining: String,
    pub extrapolating: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RaceControlMessages {
    pub messages: Vec<RaceControlMessage>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RaceControlMessage {
    pub utc: String,
    pub lap: Option<i64>,
    pub category: Category,
    pub flag: Option<Flag>,
    pub scope: Option<Scope>,
    pub message: String,
    pub sector: Option<i64>,
    pub status: Option<String>,
    pub mode: Option<String>,
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
    pub series: Vec<Series>,
    pub status_series: Vec<StatusSery>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Series {
    pub utc: String,
    pub lap: Option<i64>,
    pub qualifying_part: Option<i8>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StatusSery {
    pub utc: String,
    pub track_status: Option<String>,
    pub session_status: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TeamRadio {
    pub captures: Vec<Capture>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Capture {
    pub utc: String,
    pub racing_number: String,
    pub path: String,
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
    pub racing_number: String,
    pub line: i64,
    pub grid_pos: Option<String>,
    pub stints: Option<Vec<Stint>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Stint {
    pub lap_time: Option<String>,
    pub lap_number: Option<i64>,
    pub lap_flags: i64,
    pub compound: Compound,
    pub new: String,
    pub tyres_not_changed: String,
    pub total_laps: i64,
    pub start_laps: i64,
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
    pub line: i64,
    pub position: String, // TODO maybe issue here?
    pub show_position: bool,
    pub racing_number: String,
    pub retired: bool,
    pub in_pit: bool,
    pub pit_out: bool,
    pub stopped: bool,
    pub status: i64,
    pub number_of_laps: Option<i64>,
    pub number_of_pit_stops: Option<i64>,
    pub sectors: Vec<Sector>,
    pub speeds: Speeds,
    pub best_lap_time: BestLapTime,
    pub last_lap_time: LastLapTime,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BestLapTime {
    pub value: String,
    pub lap: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct IntervalToPositionAhead {
    pub value: String,
    pub catching: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LastLapTime {
    pub value: String,
    pub status: i64,
    pub overall_fastest: bool,
    pub personal_fastest: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Sector {
    pub stopped: bool,
    pub previous_value: Option<String>,
    pub segments: Vec<Segment>,
    pub value: String,
    pub status: i64,
    pub overall_fastest: bool,
    pub personal_fastest: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Segment {
    pub status: i64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Speeds {
    pub i1: LastLapTime,
    pub i2: LastLapTime,
    pub fl: LastLapTime,
    pub st: LastLapTime,
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
    pub line: i64,
    pub racing_number: String,
    pub personal_best_lap_time: PersonalBestLapTime,
    pub best_sectors: Vec<BestSector>,
    pub best_speeds: BestSpeeds,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BestSector {
    pub position: Option<i64>,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct BestSpeeds {
    pub i1: BestSector,
    pub i2: BestSector,
    pub fl: BestSector,
    pub st: BestSector,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PersonalBestLapTime {
    pub lap: Option<i64>,
    pub position: Option<i64>,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TopThree {
    pub lines: Vec<LineElement>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LineElement {
    pub position: String,
    pub show_position: bool,
    pub racing_number: String,
    pub tla: String,
    pub broadcast_name: String,
    pub full_name: String,
    pub team: String,
    pub team_colour: String,
    pub lap_time: String,
    pub lap_state: i64,
    pub diff_to_ahead: String,
    pub diff_to_leader: String,
    pub overall_fastest: bool,
    pub personal_fastest: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct LapSeries {
    pub racing_number: String,
    pub lap_position: Vec<String>,
}
