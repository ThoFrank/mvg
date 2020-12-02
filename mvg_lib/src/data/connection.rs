use serde::{Serialize, Deserialize};
use super::location::Location;

use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionList{
    pub connection_list: Vec<Connection>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Connection{
    zoom_notice_to: bool,
    zoom_notice_from: bool,
    from: Location,
    to: Location,
    departure: u64,
    arrival: u64,
    connection_part_list: Vec<ConnectionPart>,
    efa_ticket_ids: Vec<String>,
    server_id: u64,
    ring_from: u8,
    ring_to: u8,
    old_tarif: bool,
    banner_hash: String
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transportation{
    stops: Vec<Stop>,
    from: Location,
    to: Location,
    path: Vec<Location>,
    path_description: Vec<PathDescriptor>,
    interchange_path: Vec<Location>,
    departure: u64,
    arrival: u64,
    delay: i32,
    arr_delay: i32,
    cancelled: bool,
    product: String,
    label: String,
    server_id: String,
    destination: String,
    sev: bool,
    zoom_notice_departure: bool,
    zoom_notice_arrival: bool,
    departure_platform: String,
    departure_stop_position_number: u8,
    arrival_platform: String,
    arrival_stop_position_number: u8,
    no_changing_required: bool,
    from_id: String,
    departure_id: String,
    info_messages: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stop{
    location: Location,
    time: u64,
    delay: i32,
    arr_delay: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Footway{
    from: Location,
    to: Location,
    path: Vec<Location>,
    path_description: Vec<PathDescriptor>,
    interchange_path: Vec<()>,
    departure: u64,
    arrival: u64,
    cancelled: bool,
    zoom_notice_departure: bool,
    zoom_notice_arrival: bool,
    departure_stop_position_number: u8,
    arrival_stop_position_number: u8,
    no_changing_required: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "connectionPartType")]
#[serde(rename_all = "UPPERCASE")]
pub enum ConnectionPart{
    Transportation(Transportation),
    Footway(Footway)
}

#[derive(Serialize, Deserialize, Debug)]
struct PathDescriptor{
    from: u8,
    to: u8,
    level: i8
}

impl Connection{
    pub fn from(&self) -> &Location{
        &self.from
    }

    pub fn to(&self) -> &Location{
        &self.to
    }

    pub fn departure_time(&self) -> DateTime<Local> {
        let time =
            std::time::UNIX_EPOCH + std::time::Duration::from_millis(self.departure as u64);
        let time = DateTime::<Local>::from(time);
        time
    }
    pub fn arrival_time(&self) -> DateTime<Local> {
        let time =
            std::time::UNIX_EPOCH + std::time::Duration::from_millis(self.arrival as u64);
        let time = DateTime::<Local>::from(time);
        time
    }

    pub fn connection_parts(&self) -> &Vec<ConnectionPart>{
        &self.connection_part_list
    }
}

impl Transportation{
    pub fn from(&self) -> &Location{
        &self.from
    }

    pub fn to(&self) -> &Location{
        &self.to
    }

    pub fn product(&self) -> &String{
        &self.product
    }

    pub fn label(&self) -> &String{
        &self.label
    }
}