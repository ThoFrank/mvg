use mvg_lib::MVG;
use mvg_lib::data::location::Location;

use clap::Clap;
use colored::*;
use css_color_parser::Color as CssColor;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Stations(Stations),
    Departures(Departures),
}

/// a subcommand for fetching stations
#[derive(Clap)]
struct Stations {
    search_string: Option<String>,
}

/// a subcommand for fetching departures
#[derive(Clap)]
struct Departures {
    search_string: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let mvg = MVG::new();

    match opts.subcmd {
        SubCommand::Stations(s) => {
           print_stations(&match s.search_string{Some(s) => s, None => String::new()}, &mvg).await;
        }
        SubCommand::Departures(d) => {
            print_departures(&d.search_string, &mvg).await;
        }
    };

    Ok(())
}

async fn print_stations(search_string: &str, mvg: &MVG){
    let stations = match mvg.stations_by_name(search_string).await{
        Ok(stations) => stations,
        Err(e) => panic!("Error: {:#?}", e)
    };
    for sta in stations.iter().filter_map(|s| {
        match s {
            Location::Station(s) => Some(s),
            Location::Address(_) => None
        }
    }){
        println!("{}, {}", sta.name(), sta.place())
    }
}

async fn print_departures(search_string: &str, mvg: &MVG){
    let stations = match mvg.stations_by_id(search_string).await{
        Ok(stations) => stations,
        Err(_) => {
            match mvg.stations_by_name(search_string).await{
                Ok(stations) => stations,
                Err(_) => panic!("No Station found!")
            }
        }
    };

    let station = stations.iter().filter_map(|s| {
        match s {
            Location::Station(s) => Some(s),
            Location::Address(_) => None
        }
    }).next();

    let station = match station{
        Some(station) => station,
        None => panic!("No station found")
    };

    let departures = match mvg.departures_by_id(&station.id()).await{
        Ok(departures) => departures,
        Err(_) => panic!("Could not get Departures!")
    };
    println!("Departures at station {}, {}:", station.name(), station.place());
    for dep in departures{
        let color = dep.line_background_color.parse::<CssColor>().unwrap_or(CssColor{r: 255, g: 255, b:255, a: 1.0});
        
        print!("{}\t", dep.label().on_truecolor(color.r, color.g, color.b));
        print!("{}", dep.destination);
        for _ in 0..( 5 - (dep.destination.chars().count() / 8)){
            print!("\t");
        }
        print!("{}", dep.departure_time().format("%_H:%M"));
        println!();
    }
}
