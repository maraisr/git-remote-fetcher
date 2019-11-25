extern crate clap;
extern crate git2;

use std::borrow::Borrow;
use std::fs::{read_dir, ReadDir};
use std::io;
use std::path::Path;
use std::vec::Vec;

use clap::{App as Clap, Arg};
use git2::{FetchOptions, RemoteCallbacks, Repository};

static LOCATION_TOKEN: &str = "LOCATION";

fn main() {
	let matches = Clap::new("Git Remote Fetcher [grf]")
		.arg(
			Arg::with_name(LOCATION_TOKEN)
				.help("A location to find git locations")
				.required(true)
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

	if let Some(_repo) = is_git_repo(&start_location) {
		// TODO: Do something with root, and ignore get_all_git_dirs
	}

	let git_roots = get_all_git_directories(&start_location);

	git_roots.iter().for_each(|repo| {
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
	});
}

fn get_all_git_directories(location: &Path) -> Vec<Repository> {
	let mut fin: Vec<Repository> = Vec::new();

	walk_dir(location, &mut fin);

	#[inline]
	fn walk_dir(dir: &Path, fin: &mut Vec<Repository>) -> io::Result<()> {
		if dir.is_dir() {
			for entry in read_dir(dir)? {
				let entry = entry?;
				let path = entry.path();

				if let Some(repo) = is_git_repo(&path) {
					fin.push(repo);
				} else if path.is_dir() {
					walk_dir(&path, fin)?;
				}
			}
		}

		Ok(())
	}

	fin
}

fn is_git_repo(path: &Path) -> Option<Repository> {
	Repository::open(&path).ok()
}
