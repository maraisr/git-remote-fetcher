extern crate clap;

use std::path::Path;

use clap::{App as Clap, Arg};
use std::thread::{spawn, JoinHandle};

// Try mio
// rayon threads

static LOCATION_TOKEN: &str = "LOCATION";

struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: i8) -> ThreadPool {
        assert!(size > 0, "Thread pool size minimum size 1");

        let mut threads = Vec::<JoinHandle<()>>::with_capacity(size as usize);

        ThreadPool { threads }
    }
}

fn main() {
    let pool = ThreadPool::new(5);

    let matches = Clap::new("Git Remote Fetcher [grf]")
        .arg(
            Arg::with_name(LOCATION_TOKEN)
                .help("A location to find .git folders")
                .required(true)
                .default_value("/Users/maraisr/Sites/github/git-remote-fetcher")
                .index(1),
        )
        .get_matches();

    let start_location = Path::new(matches.value_of(LOCATION_TOKEN).unwrap());

    if !start_location.exists() {
        panic!("{:?} does not exist!", start_location)
    };

    println!("{:?}", matches.value_of(LOCATION_TOKEN).unwrap());
}
