use crate::data::location::Location;
use crate::MVG;

#[tokio::test]
async fn test_retrieve_departures() {
    let mvg = MVG::new();
    let _departures = mvg.departures_by_id(&"de:09162:2").await.unwrap();
}

#[tokio::test]
async fn test_retrieve_stations() {
    let mvg = MVG::new();
    let _stations = mvg.stations_by_name(&"Marienplatz").await.unwrap();
}

#[test]
fn test_into_location() {
    let json = "{
        \"type\": \"station\",
        \"latitude\": 48.03621598164,
        \"longitude\": 11.21876313932,
        \"id\": \"de:09188:5516\",
        \"divaId\": 5516,
        \"place\": \"Oberalting\",
        \"name\": \"Marienplatz\",
        \"hasLiveData\": false,
        \"hasZoomData\": false,
        \"products\": [
            \"BUS\"
        ],
        \"aliases\": \"\",
        \"tariffZones\": \"3|4\",
        \"lines\": {
          \"tram\": [
          ],
          \"nachttram\": [
          ],
          \"sbahn\": [
          ],
          \"ubahn\": [
          ],
          \"bus\": [
          ],
          \"nachtbus\": [
          ],
          \"otherlines\": [
          ]
        }
      }";
    let _location: Location = serde_json::from_str(json).unwrap();
}
