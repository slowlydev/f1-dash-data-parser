use base64::{engine::general_purpose, Engine};
use flate2::read::DeflateDecoder;
use serde::{de::DeserializeOwned, Deserialize, Deserializer};

use std::collections::HashMap;

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

pub fn flatten_map_to_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let map: HashMap<String, serde_json::Value> = Deserialize::deserialize(deserializer)?;
    let mut vec: Vec<T> = Vec::new();

    for (_, value) in map {
        let item = serde_json::from_value::<T>(value).map_err(serde::de::Error::custom)?;
        vec.push(item);
    }

    Ok(vec)
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
