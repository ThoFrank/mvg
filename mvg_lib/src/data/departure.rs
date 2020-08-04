use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DepartureInfo {
    #[serde(rename = "servingLines")]
    pub serving_lines: Vec<ServingLine>,
    pub departures: Vec<Departure>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServingLine {
    pub destination: String,
    pub sev: bool,
    #[serde(rename = "partialNet")]
    pub partial_net: String,
    pub product: String,
    #[serde(rename = "lineNumber")]
    pub line_number: String,
    #[serde(rename = "divaId")]
    pub diva_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Departure {
    #[serde(rename = "departureTime")]
    pub departure_time: u128,
    pub product: String,
    pub label: String,
    pub destination: String,
    pub live: bool,
    pub cancelled: bool,
    #[serde(rename = "lineBackgroundColor")]
    pub line_background_color: String,
    #[serde(rename = "departureId")]
    pub departure_id: String,
    pub sev: bool,
    pub platform: String,
    #[serde(rename = "stopPositionNumber")]
    pub stop_position_number: u8,
}

impl Departure {
    pub fn departure_time(&self) -> DateTime<Local> {
        let time =
            std::time::UNIX_EPOCH + std::time::Duration::from_millis(self.departure_time as u64);
        let time = DateTime::<Local>::from(time);
        time
    }

    pub fn label(&self) -> String{
        self.label.clone()
    }

    pub fn destination(&self) -> String{
        self.destination.clone()
    }
}