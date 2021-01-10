use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::connection::Product;

#[derive(Serialize, Deserialize, Debug)]
pub struct DepartureInfo {
    #[serde(rename = "servingLines")]
    pub serving_lines: Vec<ServingLine>,
    pub departures: Vec<Departure>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServingLine {
    destination: String,
    sev: bool,
    #[serde(rename = "partialNet")]
    partial_net: String,
    product: Product,
    #[serde(rename = "lineNumber")]
    line_number: String,
    #[serde(rename = "divaId")]
    diva_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Departure {
    #[serde(rename = "departureTime")]
    departure_time: u128,
    product: Product,
    label: String,
    destination: String,
    live: bool,
    cancelled: bool,
    #[serde(rename = "lineBackgroundColor")]
    line_background_color: String,
    #[serde(rename = "departureId")]
    departure_id: String,
    sev: bool,
    platform: String,
    #[serde(rename = "stopPositionNumber")]
    stop_position_number: u8,
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

    pub fn line_background_color(&self) -> String{
        self.line_background_color.clone()
    }
}