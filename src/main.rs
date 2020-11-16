use mvg_lib::MVG;
use mvg_lib::data::location::Location;
use mvg_lib::data::MVGError;

use clap::Clap;
use termion::{color, style};
use css_color_parser::Color as CssColor;

const STATION_NAME_MAX_CHARS: usize = 40;

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
}

/// Fetch stations
#[derive(Clap)]
struct Stations {
    /// Optional search term.
    search_term: Option<String>,
}

/// Fetch departures
#[derive(Clap)]
struct Departures {
    /// Either a station id or a station name.
    station: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let mvg = MVG::new();

    match opts.subcmd {
        SubCommand::Stations(s) => {
           print_stations(&match s.search_term{Some(s) => s, None => String::new()}, &mvg).await;
        }
        SubCommand::Departures(d) => {
            print_departures(&d.station, &mvg).await;
        }
    };

    Ok(())
}

async fn print_stations(search_string: &str, mvg: &MVG){
    let stations = match mvg.stations_by_name(search_string).await{
        Ok(stations) => stations,
        Err(e) => {
            print_mvg_err(&e);
            return;
        }
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
                Err(e) => {
                    print_mvg_err(&e);
                    return;
                }
            }
        }
    };

    // filter for stations
    let mut stations = stations.iter().filter_map(|s| {
        match s {
            Location::Station(s) => Some(s),
            _ => None
        }
    });

    let station = match stations.next(){
        Some(station) => station,
        None => {
            println!("No station found");
            return;
        }
    };

    let departures = match mvg.departures_by_id(&station.id()).await{
        Ok(departures) => departures,
        Err(e) => {
            print_mvg_err(&e);
            return;
        }
    };
    println!("Departures at station {}, {}:", station.name(), station.place());
    for dep in departures{
        let color = dep.line_background_color.parse::<CssColor>().unwrap_or(CssColor{r: 255, g: 255, b:255, a: 1.0});
        
        let adjust = |col| std::cmp::min((col as u16+32)/64, 4) as u8;

        //let color = color::Rgb(color.r, color.g, color.b);
        let color = color::AnsiValue::rgb(adjust(color.r), adjust(color.g), adjust(color.b));

        print!(
            "{}{}{}\t",
            color::Bg(color),
            dep.label(),
            style::Reset
        );
        //print!("{}\t", dep.label().on_truecolor(color.r, color.g, color.b));
        
        let destination = dep.destination();
        let dest_len = destination.chars().count();

        if dest_len > STATION_NAME_MAX_CHARS{
            print!("{}...", destination.chars().take(STATION_NAME_MAX_CHARS-3).collect::<String>());
        } else {
            print!("{}", destination);
            print!("{}", (dest_len..STATION_NAME_MAX_CHARS).map(|_| ' ').collect::<String>());
        }

        print!("{}", dep.departure_time().format("%_H:%M"));
        println!();
    }
}


fn print_mvg_err(err: &MVGError){
    println!(
        "{}Err{}: {}",
        color::Fg(color::Red),
        style::Reset,
        match err {
            MVGError::HyperError(_) => "Couldn't connect to the MVG API.",
            MVGError::JsonError(_) => "Couldn't parse API response.",
            MVGError::InvalidUri(_) => "Couldn't create valid URI.",
            _ => "Unknown Error"
        }
    )
}