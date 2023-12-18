use std::collections::HashMap;

use self::models::data::TeamRadio;

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

#[derive(Default, Debug, Clone)]
pub struct State {
    pub heartbeat: Option<models::Heartbeat>,
    pub extrapolated_clock: Option<models::ExtrapolatedClock>,
    pub top_three: Option<models::data::TopThree>,
    pub timing_stats: Option<models::data::TimingStats>,
    pub timing_app_data: Option<models::data::TimingAppData>,
    pub weather_data: Option<models::WeatherData>,
    pub track_status: Option<models::TrackStatus>,
    pub race_control_messages: Option<models::data::RaceControlMessages>,
    pub session_info: Option<models::SessionInfo>,
    pub session_data: Option<models::data::SessionData>,
    pub timing_data: Option<models::data::TimingData>,
    pub team_radio: Option<models::data::TeamRadio>,
    pub tla_rcm: Option<models::TlaRcm>,
    pub lap_count: Option<models::LapCount>,
    pub car_data: Option<models::CarData>,
    pub position: Option<models::Positions>,
    pub driver_list: Option<HashMap<String, models::data::DriverList>>,
    pub lap_series: Option<HashMap<String, models::data::LapSeries>>,

    pub pit_lane_time_collection: Option<models::updates::PitLaneTimeCollection>,
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
            lap_series: Some(data.lap_series),

            team_radio: data.team_radio,
            tla_rcm: data.tla_rcm,
            lap_count: data.lap_count,

            pit_lane_time_collection: None,
        }
    }
}

impl State {
    pub fn update_field(&mut self, update: models::Update) {
        match update {
            models::Update::Heartbeat(_, heartbeat, _) => self.heartbeat = Some(heartbeat),
            models::Update::LapCount(_, lap_count, _) => self.lap_count = Some(lap_count),
            models::Update::Positions(_, positions, _) => self.position = Some(positions),
            models::Update::CarData(_, car_data, _) => self.car_data = Some(car_data),
            models::Update::TlaRcm(_, tla_rcm, _) => self.tla_rcm = Some(tla_rcm),

            models::Update::WeatherData(_, weather_data, _) => {
                self.weather_data = Some(weather_data)
            }
            models::Update::TrackStatus(_, track_status, _) => {
                self.track_status = Some(track_status)
            }
            models::Update::SessionInfo(_, session_info, _) => {
                self.session_info = Some(session_info)
            }
            models::Update::ExtrapolatedClock(_, extrapolated_clock, _) => {
                self.extrapolated_clock = Some(extrapolated_clock)
            }

            models::Update::PitLaneTimeCollection(_, pit_lane_time_collection, _) => {
                self.pit_lane_time_collection = Some(pit_lane_time_collection)
            }

            models::Update::TopThree(_, top_three, _) => {
                // self.top_three = Some(top_three)
            }
            models::Update::TimingStats(_, timing_stats, _) => {
                // self.timing_stats = Some(timing_stats)
            }
            models::Update::TimingAppData(_, timing_app_data, _) => {
                // self.timing_app_data = Some(timing_app_data)
            }
            models::Update::TimingData(_, timing_data, _) => {
                // self.timing_data = Some(timing_data)
            }

            models::Update::TeamRadio(_, mut team_radio, _) => {
                let prev_team_radio = std::mem::take(&mut self.team_radio);
                self.team_radio = match prev_team_radio {
                    Some(mut prev_team_radio) => {
                        prev_team_radio.captures.append(&mut team_radio.captures);
                        Some(prev_team_radio)
                    }
                    None => Some(team_radio.into()),
                }
            }

            models::Update::RaceControlMessages(_, race_control_messages, _) => {
                // self.race_control_messages = Some(race_control_messages)
            }
            models::Update::LapSeries(_, lap_series, _) => {
                // self.lap_series = Some(lap_series)
            }
            models::Update::SessionData(_, session_data, _) => {
                // self.session_data = Some(session_data)
            }

            models::Update::DriverList(_, _, _) => {} // this case does simply not exsist
        }
    }
}
