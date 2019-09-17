extern crate clap;
extern crate walkdir;

use std::path::Path;

use clap::{App as Clap, Arg};

// Try mio
// rayon threads

static LOCATION_TOKEN: &str = "LOCATION";

fn main() {
    let matches = Clap::new("Git Remote Fetcher [grf]")
        .arg(
            Arg::with_name(LOCATION_TOKEN)
                .help("A location to find .git folders")
                .required(true)
                .default_value("C:\\Users\\marais\\Sites")
                .index(1),
        )
        .get_matches();

    let start_location = Path::new(matches.value_of(LOCATION_TOKEN).unwrap());

    if !start_location.exists() {
        panic!("{:?} does not exist!", start_location)
    };

    println!("{:?}", matches.value_of(LOCATION_TOKEN).unwrap());
}
