use std::collections::HashMap;

pub mod deserializer;
pub mod models;

#[derive(Debug)]
pub enum ParsedMessage {
    Update(Vec<models::Update>),
    Replay(models::Data),
    Empty,
}

pub fn parse_message(message: String) -> ParsedMessage {
    let socket_message: models::SocketMessage =
        serde_json::from_str::<models::SocketMessage>(&message).unwrap();

    if let Some(updates) = socket_message.m {
        let mut vec: Vec<models::Update> = Vec::new();

        if updates.len() < 1 {
            return ParsedMessage::Empty;
        };

        for update in updates {
            vec.push(update.a);
        }

        return ParsedMessage::Update(vec);
    };

    if let Some(replay) = socket_message.r {
        return ParsedMessage::Replay(replay);
    }

    ParsedMessage::Empty
}

#[derive(Default)]
pub struct State {
    pub heartbeat: Option<models::data::Heartbeat>,
    pub extrapolated_clock: Option<models::data::ExtrapolatedClock>,
    pub top_three: Option<models::data::TopThree>,
    pub timing_stats: Option<models::data::TimingStats>,
    pub timing_app_data: Option<models::data::TimingAppData>,
    pub weather_data: Option<models::data::WeatherData>,
    pub track_status: Option<models::data::TrackStatus>,
    pub race_control_messages: Option<models::data::RaceControlMessages>,
    pub session_info: Option<models::data::SessionInfo>,
    pub session_data: Option<models::data::SessionData>,
    pub timing_data: Option<models::data::TimingData>,
    pub team_radio: Option<models::data::TeamRadio>,
    pub tla_rcm: Option<models::data::TlaRcm>,
    pub lap_count: Option<models::data::LapCount>,
    pub driver_list: Option<HashMap<String, models::data::DriverList>>,
    pub car_data: Option<models::data::CarData>,
    pub position: Option<models::data::Positions>,
}

impl From<models::Data> for State {
    fn from(data: models::Data) -> Self {
        State {
            heartbeat: Some(data.heartbeat),
            extrapolated_clock: Some(data.extrapolated_clock),
            top_three: Some(data.top_three),
            timing_stats: Some(data.timing_stats),
            timing_app_data: Some(data.timing_app_data),
            weather_data: Some(data.weather_data),
            track_status: Some(data.track_status),
            race_control_messages: Some(data.race_control_messages),
            session_info: Some(data.session_info),
            session_data: Some(data.session_data),
            timing_data: Some(data.timing_data),
            driver_list: Some(data.driver_list),
            car_data: Some(data.car_data),
            position: Some(data.position),

            team_radio: data.team_radio,
            tla_rcm: data.tla_rcm,
            lap_count: data.lap_count,
        }
    }
}
