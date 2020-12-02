// source of urls: https://github.com/leftshift/python_mvg_api/blob/master/mvg_api/__init__.py

/// Generate URL to query stations by name
#[allow(dead_code)]
pub fn query_url_name(name: &str) -> String {
    format!(
        "https://www.mvg.de/api/fahrinfo/location/queryWeb?q={}",
        name
    )
}
/// Generate URL to query station by id
#[allow(dead_code)]
pub fn query_url_id(id: &str) -> String {
    format!("https://www.mvg.de/api/fahrinfo/departure/{}?footway=0", id)
}
/// Generate URL to query departures by station id
#[allow(dead_code)]
pub fn departure_url(id: &str) -> String {
    format!("https://www.mvg.de/api/fahrinfo/departure/{}?footway=0", id)
}
/// Generate URL to query nearby stations
#[allow(dead_code)]
pub fn nearby_url(lat: f64, lon: f64) -> String {
    format!(
        "https://www.mvg.de/api/fahrinfo/location/nearby?latitude={}&longitude={}",
        lat, lon
    )
}
/// Generate URL to query routing info
#[allow(dead_code)]
pub fn routing_url(from_id: &str, to_id: &str) -> String {
    let mut options = Vec::new();
    options.push(format!("fromStation={}", from_id));
    options.push(format!("toStation={}", to_id));
    /* Possible other options:
    options.push(format!("time={}", time));
    options.push(format!("arrival={}", true));
    options.push(format!("maxTravelTimeFootwayToStation={}", 60));
    options.push(format!("maxTravelTimeFootwayToDestination={}", 60));
    options.push(format!("changeLimit={}", 5));
    options.push(String::from("transportTypeUnderground=false"));
    options.push(String::from("transportTypeBus=false"));
    options.push(String::from("transportTypeTram=false"));
    options.push(String::from("transportTypeSBahn=false"));
    */
    let options = options.join("&");
    format!("https://www.mvg.de/api/fahrinfo/routing/?{}", options)
}
/// Generate URL to query interruptions
#[allow(dead_code)]
pub fn interruptions_url() -> String {
    String::from("https://www.mvg.de/.rest/betriebsaenderungen/api/interruptions")
}
