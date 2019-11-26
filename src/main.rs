extern crate clap;
extern crate git2;

use std::fs::read_dir;
use std::io;
use std::io::Write;
use std::path::Path;
use std::vec::Vec;

use clap::{App as Clap, Arg};
use git2::{FetchOptions, Remote, RemoteCallbacks, Repository};

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
		println!(
			"\rUpdating {}",
			repo.workdir()
				.unwrap()
				.strip_prefix(&start_location)
				.unwrap()
				.to_str()
				.unwrap()
		);

		repo.remotes()
			.unwrap()
			.iter()
			.filter_map(|x| repo.find_remote(x.unwrap()).ok())
			.for_each(move |mut remote| {
				fetch_remote_for_repo(&mut remote);
			});
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

fn fetch_remote_for_repo(remote: &mut Remote) -> std::option::Option<()> {
	let remote_name = remote.name()?;

	print!("\r\t[{}] fetching...", remote_name);
	io::stdout().flush().unwrap();

	let mut cb = RemoteCallbacks::new();

	// Took inspiration from: https://github.com/rust-lang/cargo/blob/a41c8eae701c33abd327d13ff5c057389d8801b9/src/cargo/sources/git/utils.rs#L410-L624
	cb.credentials(|url, username, cred_type| {
		if cred_type.is_ssh_key() {
			return git2::Cred::ssh_key_from_agent(username.unwrap());
		}

		if cred_type.is_username() {
			return git2::Cred::username(username.unwrap());
		}

		if cred_type.is_user_pass_plaintext() {
			let cfg = git2::Config::open_default().unwrap();

			return git2::Cred::credential_helper(&cfg, url, username);
		}

		if cred_type.is_default() {
			return git2::Cred::default();
		}

		Err(git2::Error::from_str("no authentication available"))
	});

	cb.transfer_progress(|stats| {
		print!(
			"received {}/{} objects ({}) in {} bytes\r",
			stats.received_objects(),
			stats.total_objects(),
			stats.indexed_objects(),
			stats.received_bytes()
		);
		io::stdout().flush().unwrap();
		true
	});

	let mut fo = FetchOptions::new();
	fo.remote_callbacks(cb);

	match remote.download(&[], Some(&mut fo)) {
		Ok(_) => {
			{
				let stats = remote.stats();
				if stats.received_bytes() > 0 {
					println!(
						"\r\t[{}] received {} objects at {} bytes",
						remote.name()?,
						stats.total_objects(),
						stats.received_bytes()
					);
				} else {
					println!("\r\t[{}] up to date", remote.name()?);
				}
			}

			remote.disconnect();

			remote
				.update_tips(None, true, git2::AutotagOption::Unspecified, None)
				.unwrap();
		}
		Err(e) => {
			eprintln!(
				"\r\tfailed fetching \"{}\" with message: {:?}",
				remote.name()?,
				e.message()
			);
		}
	}

	Some(())
}
