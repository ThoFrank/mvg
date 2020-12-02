use mvg_lib::data::location;
use mvg_lib::data::MVGError;
use mvg_lib::MVG;
use mvg_lib::data::connection;

use clap::Clap;
use css_color_parser::Color as CssColor;
use lazy_static::lazy_static;
use termion::{color, style};

mod conf;
use conf::Config;

const STATION_NAME_MAX_CHARS: usize = 40;

lazy_static! {
    static ref CONFIG: Config = conf::load_config(&conf::DEFAULT_LOCATION);
    static ref OPTS: Opts = Opts::parse();
}

/// Command line interface to Munich's public transportation service.
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
    Connections(Connections)
}

/// Fetch stations
#[derive(Clap)]
struct Stations {
    /// Optional search term.
    search_term: Option<String>
}

/// Fetch departures
#[derive(Clap)]
struct Departures {
    /// Either a station id or a station name.
    station: Option<String>,
}

/// search connections
#[derive(Clap)]
struct Connections{
    /// departure station
    from_station: String,
    /// destination station
    to_station: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mvg = MVG::new();

    match &OPTS.subcmd {
        SubCommand::Stations(s) => {
            print_stations(
                &match &s.search_term {
                    Some(s) => String::from(s),
                    None => String::new(),
                },
                &mvg,
            )
            .await;
        }
        SubCommand::Departures(d) => {
            let station = d.station.as_ref().or(CONFIG.default_station.as_ref());
            if let Some(station) = station {
                print_departures(&station, &mvg).await;
            } else {
                println!("Please provide a station!");
            }
        }
        SubCommand::Connections(c) => {
            print_connections(&c.from_station, &c.to_station, &mvg).await;
        }
    };

    Ok(())
}

async fn print_stations(search_string: &str, mvg: &MVG) {
    let stations = match mvg.stations_by_name(search_string).await {
        Ok(stations) => stations,
        Err(e) => {
            print_mvg_err(&e);
            return;
        }
    };
    for sta in stations.iter().filter_map(|s| match s {
        location::Location::Station(s) => Some(s),
        location::Location::Address(_) => None,
        location::Location::Location(_) => None
    }) {
        println!("{}, {}, {}", sta.name(), sta.place(), sta.id())
    }
}

async fn print_departures(search_string: &str, mvg: &MVG) {
    let stations = match mvg.stations_by_id(search_string).await {
        Ok(stations) => stations,
        Err(_) => match mvg.stations_by_name(search_string).await {
            Ok(stations) => stations,
            Err(e) => {
                print_mvg_err(&e);
                return;
            }
        },
    };

    // filter for stations
    let mut stations = stations.iter().filter_map(|s| match s {
        location::Location::Station(s) => Some(s),
        _ => None,
    });

    let station = match stations.next() {
        Some(station) => station,
        None => {
            println!("No station found");
            return;
        }
    };

    let departures = match mvg.departures_by_id(&station.id()).await {
        Ok(departures) => departures,
        Err(e) => {
            print_mvg_err(&e);
            return;
        }
    };
    println!(
        "Departures at station {}, {}:",
        station.name(),
        station.place()
    );
    for dep in departures {
        let color = dep
            .line_background_color()
            .parse::<CssColor>()
            .unwrap_or(CssColor {
                r: 255,
                g: 255,
                b: 255,
                a: 1.0,
            });

        let adjust = |col| std::cmp::min((col as u16 + 32) / 64, 4) as u8;

        //let color = color::Rgb(color.r, color.g, color.b);
        let color = color::AnsiValue::rgb(adjust(color.r), adjust(color.g), adjust(color.b));

        print!("{}{}{}\t", color::Bg(color), dep.label(), style::Reset);
        //print!("{}\t", dep.label().on_truecolor(color.r, color.g, color.b));

        let destination = dep.destination();
        let dest_len = destination.chars().count();

        if dest_len > STATION_NAME_MAX_CHARS {
            print!(
                "{}...",
                destination
                    .chars()
                    .take(STATION_NAME_MAX_CHARS - 3)
                    .collect::<String>()
            );
        } else {
            print!("{}", destination);
            print!(
                "{}",
                (dest_len..STATION_NAME_MAX_CHARS)
                    .map(|_| ' ')
                    .collect::<String>()
            );
        }

        print!("{}", dep.departure_time().format("%_H:%M"));
        println!();
    }
}

async fn print_connections(from: &str, to: &str, mvg: &MVG){
    let from: Vec<mvg_lib::data::location::Station> = match mvg.stations_by_name(from).await{
        Ok(stations) => stations,
        Err(e) => {
            print_mvg_err(&e);
            return;
        }
    }.into_iter().filter_map(|s| match s {
        location::Location::Station(s) => Some(s),
        _ => None,
    }).collect();

    let from = from.first().unwrap();

    let to: Vec<mvg_lib::data::location::Station> = match mvg.stations_by_name(to).await{
        Ok(stations) => stations,
        Err(e) => {
            print_mvg_err(&e);
            return;
        }
    }.into_iter().filter_map(|s| match s {
        location::Location::Station(s) => Some(s),
        _ => None,
    }).collect();

    let to = to.first().unwrap();

    let connections = mvg.connections(&from.id(), &to.id()).await;
    if let Err(e) = &connections {
        print_mvg_err(e);
        println!("{:#?}", e);
        return;
    }
    let connections = connections.unwrap();
    for (i, con) in connections.into_iter().enumerate(){
        println!{"Connection {}", i};
        for con_part in con.connection_parts(){
            match con_part{
                connection::ConnectionPart::Footway(_fw) => {
                    println!("Run!");
                }
                connection::ConnectionPart::Transportation(tp) => {
                    if let location::Location::Station(from) = tp.from(){
                        if let location::Location::Station(to) = tp.to(){
                            println!("Take {} from {} to {}", tp.label(), from.name(), to.name());
                        }
                    }
                }
            }
        }
    }
}

fn print_mvg_err(err: &MVGError) {
    println!(
        "{}Err{}: {}",
        color::Fg(color::Red),
        style::Reset,
        match err {
            MVGError::HyperError(_) => "Couldn't connect to the MVG API.",
            MVGError::JsonError(_) => "Couldn't parse API response.",
            MVGError::InvalidUri(_) => "Couldn't create valid URI.",
            _ => "Unknown Error",
        }
    )
}
