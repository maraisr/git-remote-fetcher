extern crate clap;
extern crate git2;
extern crate num_cpus;
extern crate rayon;
extern crate scan_dir;

use std::path::Path;
use std::vec::Vec;

use clap::{App as Clap, Arg};
use git2::BranchType::Remote;
use git2::{FetchOptions, RemoteCallbacks, Repository};
use rayon::ThreadPoolBuilder;
use scan_dir::ScanDir;
use std::fs::DirEntry;
use std::io::{self, Write};

static LOCATION_TOKEN: &str = "LOCATION";

fn main() {
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build()
        .unwrap();

    let matches = Clap::new("Git Remote Fetcher [grf]")
        .arg(
            Arg::with_name(LOCATION_TOKEN)
                .help("A location to find git locations")
                .required(true)
                .default_value(if cfg!(windows) {
                    "C:\\Users\\marais\\Sites\\"
                } else {
                    "/Users/maraisr/Sites/github/"
                })
                .index(1),
        )
        .get_matches();

    let start_location = Path::new(
        matches
            .value_of(LOCATION_TOKEN)
            .expect("The location provided could'nt be resolved."),
    );

    if !start_location.exists() {
        panic!("{:?} does not exist!", start_location)
    };

    // TODO: Is start a git repo?

    let test = get_all_git_directories(&start_location);

    test.iter().for_each(|repo| {
        //pool.install(|| {
        repo.remotes().unwrap().iter().for_each(|x| {
            let mut remote = repo.find_remote(x.unwrap()).unwrap();

            println!("Fetching {:?} at {:?}", remote.name().unwrap(), repo.path());

            let mut cb = RemoteCallbacks::new();

            cb.credentials(|_url, username, _cred_type| {
                git2::Cred::ssh_key_from_agent(username.unwrap_or("git"))
            });

            let mut fo = FetchOptions::new();
            fo.remote_callbacks(cb);
            remote.download(&[], Some(&mut fo)).unwrap();
            remote.disconnect();
            remote
                .update_tips(None, true, git2::AutotagOption::Unspecified, None)
                .unwrap();
        })
        //})
    });
}

fn get_all_git_directories(location: &Path) -> Vec<Repository> {
    // TODO: Look at this https://play.rust-lang.org/?gist=89ffaf05037e91c149e3d6a4b5352462&version=stable
    ScanDir::dirs()
        .walk(location, |mut iter| {
            let mut fin: Vec<Repository> = Vec::new();

            while let Some((entry, _name)) = iter.next() {
                let maybe_repo = Repository::open(entry.path().as_path());

                if !maybe_repo.is_ok() {
                    iter.exit_current_dir();
                } else {
                    fin.push(maybe_repo.unwrap());
                }
            }

            fin
        })
        .unwrap()
}
