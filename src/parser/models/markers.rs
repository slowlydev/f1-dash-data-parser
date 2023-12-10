use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum HeartbeatMarker {
    Heartbeat,
}

#[derive(Debug, Deserialize, Clone)]
pub enum ExtrapolatedClockMarker {
    ExtrapolatedClock,
}

#[derive(Debug, Deserialize, Clone)]
pub enum TopThreeMarker {
    TopThree,
}

#[derive(Debug, Deserialize, Clone)]
pub enum TimingStatsMarker {
    TimingStats,
}

#[derive(Debug, Deserialize, Clone)]
pub enum TimingAppDataMarker {
    TimingAppData,
}

#[derive(Debug, Deserialize, Clone)]
pub enum WeatherDataMarker {
    WeatherData,
}

#[derive(Debug, Deserialize, Clone)]
pub enum TrackStatusMarker {
    TrackStatus,
}

#[derive(Debug, Deserialize, Clone)]
pub enum RaceControlMessagesMarker {
    RaceControlMessages,
}

#[derive(Debug, Deserialize, Clone)]
pub enum SessionInfoMarker {
    SessionInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub enum SessionDataMarker {
    SessionData,
}

#[derive(Debug, Deserialize, Clone)]
pub enum LapCountMarker {
    LapCount,
}

#[derive(Debug, Deserialize, Clone)]
pub enum TimingDataMarker {
    TimingData,
    TimingDataF1,
}

#[derive(Debug, Deserialize, Clone)]
pub enum TeamRadioMarker {
    TeamRadio,
}

#[derive(Debug, Deserialize, Clone)]
pub enum TlaRcmMarker {
    TlaRcm,
}

#[derive(Debug, Deserialize, Clone)]
pub enum PitLaneTimeCollectionMarker {
    PitLaneTimeCollection,
}

#[derive(Debug, Deserialize, Clone)]
pub enum LapSeriesMarker {
    LapSeries,
}

#[derive(Debug, Deserialize, Clone)]
pub enum DriverListMarker {
    DriverList,
}

#[derive(Debug, Deserialize, Clone)]
pub enum CarDataMarker {
    #[serde(rename = "CarData.z")]
    CarData,
}

#[derive(Debug, Deserialize, Clone)]
pub enum PositionsMarker {
    #[serde(rename = "Position.z")]
    Positions,
}
