use base64::{engine::general_purpose, Engine};
use flate2::read::DeflateDecoder;
use serde::{de::DeserializeOwned, Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;

use super::models::{self, data, DataType};

pub fn kf_remover<'de, D, T>(deserializer: D) -> Result<HashMap<String, T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let mut map = HashMap::new();
    let mut raw_map: HashMap<String, serde_json::Value> = Deserialize::deserialize(deserializer)?;
    raw_map.remove("_kf");
    for (key, value) in raw_map {
        if let Ok(item) = serde_json::from_value::<T>(value) {
            map.insert(key, item);
        }
    }
    Ok(map)
}

pub fn inflate_zlib<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let decoded: Vec<u8> = general_purpose::STANDARD
        .decode(s)
        .map_err(serde::de::Error::custom)?;
    let decoder: DeflateDecoder<&[u8]> = DeflateDecoder::new(&decoded[..]);
    let data: T = serde_json::from_reader(decoder).map_err(serde::de::Error::custom)?;
    Ok(data)
}

pub fn route_message_type<'de, D>(
    deserializer: D,
) -> Result<(String, models::DataType, String), D::Error>
where
    D: Deserializer<'de>,
{
    let value: (String, Value, String) = Deserialize::deserialize(deserializer)?;

    let data: DataType = match value.0.as_str() {
        "Heartbeat" => serde_json::from_value::<data::Heartbeat>(value.1).map(DataType::Heartbeat),
        "ExtrapolatedClock" => serde_json::from_value::<data::ExtrapolatedClock>(value.1)
            .map(DataType::ExtrapolatedClock),
        "TopThree" => serde_json::from_value::<data::TopThree>(value.1).map(DataType::TopThree),
        "TimingStats" => {
            serde_json::from_value::<data::TimingStats>(value.1).map(DataType::TimingStats)
        }
        "TimingAppData" => {
            serde_json::from_value::<data::TimingAppData>(value.1).map(DataType::TimingAppData)
        }
        "WeatherData" => {
            serde_json::from_value::<data::WeatherData>(value.1).map(DataType::WeatherData)
        }
        "TrackStatus" => {
            serde_json::from_value::<data::TrackStatus>(value.1).map(DataType::TrackStatus)
        }
        "RaceControlMessages" => serde_json::from_value::<data::RaceControlMessages>(value.1)
            .map(DataType::RaceControlMessages),
        "SessionInfo" => {
            serde_json::from_value::<data::SessionInfo>(value.1).map(DataType::SessionInfo)
        }
        "SessionData" => {
            serde_json::from_value::<data::SessionData>(value.1).map(DataType::SessionData)
        }
        "LapCount" => serde_json::from_value::<data::LapCount>(value.1).map(DataType::LapCount),
        "TimingData" => {
            serde_json::from_value::<data::TimingData>(value.1).map(DataType::TimingData)
        }
        "TimingDataF1" => {
            serde_json::from_value::<data::TimingData>(value.1).map(DataType::TimingData)
        }
        "TeamRadio" => serde_json::from_value::<data::TeamRadio>(value.1).map(DataType::TeamRadio),

        "TlaRcm" => serde_json::from_value::<data::TlaRcm>(value.1).map(DataType::TlaRcm),

        "DriverList" => serde_json::from_value::<HashMap<String, data::DriverList>>(value.1)
            .map(DataType::DriverList),
        "CarData.z" => inflate_zlib::<Value, data::CarData>(value.1).map(DataType::CarData),
        "Position.z" => inflate_zlib::<Value, data::Positions>(value.1).map(DataType::Positions),
        _ => Err(serde::de::Error::custom("Unknown message type")),
    }
    .map_err(serde::de::Error::custom)?;

    Ok((value.0, data, value.2))
}
