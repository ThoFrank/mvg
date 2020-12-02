pub mod data;
mod query;

#[cfg(test)]
mod test;

use hyper::{body::HttpBody as _, client::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use data::{MVGError};
use data::location::{Location, Locations};
use data::departure::{Departure, DepartureInfo};
use data::connection::{ConnectionList, Connection};

pub struct MVG {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl MVG {
    pub fn new() -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        MVG { client: client }
    }

    pub async fn stations_by_name(&self, search: &str) -> Result<Vec<Location>, MVGError> {
        let search = utf8_percent_encode(search, NON_ALPHANUMERIC).to_string();
        let url = query::query_url_name(&search);
        let url = url.parse::<hyper::Uri>()?;

        let mut res = self.client.get(url).await?;

        if res.status() != http::status::StatusCode::OK {
            return Err(MVGError::ArgumentError("No response".to_string()));
        }
        let mut json = String::new();
        while let Some(next) = res.data().await {
            let chunk = next?;
            json += &String::from_utf8_lossy(&chunk);
        }

        let locations_raw: Locations = serde_json::from_str(&json)?;
        Ok(locations_raw.locations)
    }

    pub async fn stations_by_id(&self, id: &str) -> Result<Vec<Location>, MVGError> {
        let url = query::query_url_id(id);
        let url = url.parse::<hyper::Uri>()?;

        let mut res = self.client.get(url).await?;

        if res.status() != http::status::StatusCode::OK {
            return Err(MVGError::ArgumentError("No response".to_string()));
        }
        let mut json = String::new();
        while let Some(next) = res.data().await {
            let chunk = next?;
            json += &String::from_utf8_lossy(&chunk);
        }

        let locations_raw: Locations = serde_json::from_str(&json)?;
        Ok(locations_raw.locations)
    }

    pub async fn departures_by_id(&self, station_id: &str) -> Result<Vec<Departure>, MVGError> {
        let url: String = query::departure_url(station_id);
        let url = url.parse::<hyper::Uri>()?;

        let mut res = self.client.get(url).await?;

        if res.status() != http::status::StatusCode::OK {
            return Err(MVGError::ArgumentError(
                format!("No valid station id: {}", station_id)
            ));
        }
        let mut json = String::new();
        while let Some(next) = res.data().await {
            let chunk = next?;
            json += &String::from_utf8_lossy(&chunk);
        }
        let departure_info: DepartureInfo = serde_json::from_str(&json)?;
        Ok(departure_info.departures)
    }

    pub async fn connections(&self, from_id: &str, to_id: &str) -> Result<Vec<Connection>, MVGError>{
        let url = query::routing_url(from_id, to_id);
        let url = url.parse::<hyper::Uri>()?;

        let mut res = self.client.get(url).await?;

        if res.status() != http::status::StatusCode::OK {
            return Err(MVGError::ArgumentError(
                format!("No valid station ids: {} - {}", from_id, to_id)
            ));
        }
        let mut json = String::new();
        while let Some(next) = res.data().await {
            let chunk = next?;
            json += &String::from_utf8_lossy(&chunk);
        }
        let connections: ConnectionList = serde_json::from_str(&json)?;
        let connections = connections.connection_list;
        Ok(connections)
    }
}

