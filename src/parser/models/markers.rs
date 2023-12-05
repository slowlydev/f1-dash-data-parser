use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum HeartbeatMarker {
    Heartbeat,
}

#[derive(Debug, Deserialize)]
pub enum ExtrapolatedClockMarker {
    ExtrapolatedClock,
}

#[derive(Debug, Deserialize)]
pub enum TopThreeMarker {
    TopThree,
}

#[derive(Debug, Deserialize)]
pub enum TimingStatsMarker {
    TimingStats,
}

#[derive(Debug, Deserialize)]
pub enum TimingAppDataMarker {
    TimingAppData,
}

#[derive(Debug, Deserialize)]
pub enum WeatherDataMarker {
    WeatherData,
}

#[derive(Debug, Deserialize)]
pub enum TrackStatusMarker {
    TrackStatus,
}

#[derive(Debug, Deserialize)]
pub enum RaceControlMessagesMarker {
    RaceControlMessages,
}

#[derive(Debug, Deserialize)]
pub enum SessionInfoMarker {
    SessionInfo,
}

#[derive(Debug, Deserialize)]
pub enum SessionDataMarker {
    SessionData,
}

#[derive(Debug, Deserialize)]
pub enum LapCountMarker {
    LapCount,
}

#[derive(Debug, Deserialize)]
pub enum TimingDataMarker {
    TimingData,
    TimingDataF1,
}

#[derive(Debug, Deserialize)]
pub enum TeamRadioMarker {
    TeamRadio,
}

#[derive(Debug, Deserialize)]
pub enum TlaRcmMarker {
    TlaRcm,
}

#[derive(Debug, Deserialize)]
pub enum DriverListMarker {
    DriverList,
}

#[derive(Debug, Deserialize)]
pub enum CarDataMarker {
    #[serde(rename = "CarData.z")]
    CarData,
}

#[derive(Debug, Deserialize)]
pub enum PositionsMarker {
    #[serde(rename = "Position.z")]
    Positions,
}
