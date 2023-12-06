use base64::{engine::general_purpose, Engine};
use flate2::read::DeflateDecoder;
use serde::{de::DeserializeOwned, Deserialize, Deserializer};
use serde_json::Value;

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
        let item = serde_json::from_value::<T>(value).map_err(serde::de::Error::custom)?;
        map.insert(key, item);
    }
    Ok(map)
}

pub fn flatten_map_to_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let map: HashMap<String, T> = Deserialize::deserialize(deserializer)?;
    let mut vec: Vec<T> = Vec::new();

    for (_, value) in map {
        // let item = serde_json::from_value::<T>(value).map_err(serde::de::Error::custom)?;
        vec.push(value);
    }

    Ok(vec)
}

pub fn flatten_map_to_vec_optional<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let optional_map: Option<HashMap<String, T>> = Deserialize::deserialize(deserializer)?;

    if let Some(map) = optional_map {
        let mut vec: Vec<T> = Vec::new();
        for (_, value) in map {
            vec.push(value);
        }
        return Ok(Some(vec));
    }

    Ok(None)
}

#[derive(Deserialize)]
#[serde(untagged)]
enum VecOrMap<T> {
    Vec(Vec<T>),
    Map(HashMap<String, T>),
}

pub fn map_or_vec_to_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let vec_or_map: VecOrMap<T> = Deserialize::deserialize(deserializer)?;
    let res: Vec<T> =
        match vec_or_map {
            VecOrMap::Vec(vec) => vec,
            VecOrMap::Map(map) => {
                let mut vec: Vec<T> = Vec::new();
                for (_, value) in map {
                    vec.push(value);
                }
                vec
            }
        };

    Ok(res)
}

pub fn map_or_vec_to_vec_optional<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let vec_or_map_option: Option<VecOrMap<T>> = Deserialize::deserialize(deserializer)?;

    if let Some(vec_or_map) = vec_or_map_option {
        let res: Vec<T> = match vec_or_map {
            VecOrMap::Vec(vec) => vec,
            VecOrMap::Map(map) => {
                let mut vec: Vec<T> = Vec::new();
                for (_, value) in map {
                    vec.push(value);
                }
                vec
            }
        };

        return Ok(Some(res));
    };

    Ok(None)
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
