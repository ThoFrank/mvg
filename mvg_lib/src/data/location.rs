use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Station {
    latitude: f64,
    longitude: f64,
    id: String,
    #[serde(rename = "divaId")]
    diva_id: usize,
    place: String,
    name: String,
    #[serde(rename = "hasLiveData")]
    has_live_data: bool,
    #[serde(rename = "hasZoomData")]
    has_zoom_data: bool,
    products: Vec<String>,
    aliases: Option<String>,
    link: Option<String>,
    #[serde(rename = "tariffZones")]
    tariff_zones: String,
    lines: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address{
    latitude: f64,
    longitude: f64,
    place: String,
    street: String,
    poi: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position{
    latitude: f32,
    longitude: f32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Location{
    #[serde(rename = "station")]
    Station(Station),
    #[serde(rename = "address")]
    Address(Address),
    #[serde(rename = "location")]
    Location(Position)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Locations {
    pub locations: Vec<Location>,
}

impl Station{
    pub fn id(&self) -> String{
        self.id.clone()
    }

    pub fn name(&self) -> String{
        self.name.clone()
    }

    pub fn place(&self) -> String{
        self.place.clone()
    }
}