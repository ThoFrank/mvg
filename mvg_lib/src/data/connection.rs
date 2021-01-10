use serde::{Serialize, Deserialize};
use super::location::Location;

use chrono::{DateTime, Local};

/// returned by the mvg api
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionList{
    pub connection_list: Vec<Connection>
}

/// Desciption of one time-dependant connection
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

/// Transportation from one to another location by one product of public traffic
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
    product: Product,
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

/// A stop during a transportation
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stop{
    location: Location,
    time: u64,
    delay: i32,
    arr_delay: i32,
}

/// Part of a connection which has to be walked
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

/// Representing one part of a connection.
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Product{
    SBahn,
    UBahn,
    Bus,
    Bahn,
    Tram
}

impl Connection{
    /// the starting location
    pub fn from(&self) -> &Location{
        &self.from
    }

    /// the destination
    pub fn to(&self) -> &Location{
        &self.to
    }

    /// start time
    pub fn departure_time(&self) -> DateTime<Local> {
        let time =
            std::time::UNIX_EPOCH + std::time::Duration::from_millis(self.departure as u64);
        let time = DateTime::<Local>::from(time);
        time
    }

    /// end time
    pub fn arrival_time(&self) -> DateTime<Local> {
        let time =
            std::time::UNIX_EPOCH + std::time::Duration::from_millis(self.arrival as u64);
        let time = DateTime::<Local>::from(time);
        time
    }

    /// list of different connection parts
    pub fn connection_parts(&self) -> &Vec<ConnectionPart>{
        &self.connection_part_list
    }
}

impl Transportation{
    /// starting location
    pub fn from(&self) -> &Location{
        &self.from
    }

    /// destination
    pub fn to(&self) -> &Location{
        &self.to
    }

    /// transporting product (e.g. UBAHN, SBAHN)
    pub fn product(&self) -> &Product{
        &self.product
    }

    /// label of transporting product (e.g. U6, S7)
    pub fn label(&self) -> &String{
        &self.label
    }
}