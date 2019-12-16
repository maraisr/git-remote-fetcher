use std::fs::read_dir;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;
use std::vec::Vec;

use clap::{App as Clap, Arg};
use git2::{FetchOptions, Remote, RemoteCallbacks, Repository};

type Result<T> = std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() {
	if let Err(_err) = run() {
		::std::process::exit(1);
	}
}

fn run() -> Result<()> {
	let matches = Clap::new("Git Remote Fetcher [git-remote-fetcher]")
		.about("A utility that fetches all remotes for all git roots south of a given location.")
		.arg(
			Arg::with_name("location")
				.help("A location to find git locations")
				.required(true),
		)
		.get_matches();

	let start_location = Path::new(
		matches
			.value_of("location")
			.expect("The location provided could'nt be resolved."),
	);

	if !start_location.exists() {
		panic!("{:?} does not exist!", start_location)
	};

	let mut roots: Vec<PathBuf> = Vec::new();

	if is_git_repo(&start_location) {
		roots.push(start_location.to_path_buf());
	} else {
		roots.extend(get_all_git_directories(&start_location));
	}

	run_fetchers_at(roots)
}

fn run_fetchers_at(roots: Vec<PathBuf>) -> Result<()> {
	let mut handlers = Vec::new();

	for x in roots {
		handlers.push(thread::spawn(move || {
			let repo = Repository::open(&x).unwrap();
			fetch_all_remotes_for_repo(&repo).unwrap();
		}));
	}

	for x in handlers {
		let _ = x.join();
	}

	Ok(()) // TODO:  Maybe collect errors if there are any
}

fn get_all_git_directories(location: &Path) -> Vec<PathBuf> {
	let mut fin: Vec<PathBuf> = Vec::new();

	walk_dir(location, &mut fin);

	#[inline]
	fn walk_dir(dir: &Path, fin: &mut Vec<PathBuf>) -> io::Result<()> {
		if dir.is_dir() {
			for entry in read_dir(dir)? {
				let entry = entry?;
				let path = entry.path();

				if is_git_repo(&path) {
					fin.push(path.to_owned());
				} else if path.is_dir() {
					walk_dir(&path, fin)?;
				}
			}
		}

		Ok(())
	}

	fin
}

fn is_git_repo(path: &Path) -> bool {
	Repository::open(&path).is_ok()
}

fn fetch_all_remotes_for_repo(repo: &Repository) -> Result<()> {
	repo.remotes()?
		.iter()
		.filter_map(|x| repo.find_remote(x?).ok())
		.for_each(move |mut remote| {
			fetch_remote_for_repo(&mut remote, repo.workdir().unwrap());
		});

	Ok(())
}

fn fetch_remote_for_repo(remote: &mut Remote, repo_path: &Path) -> std::option::Option<()> {
	let remote_name = remote.name()?;

	println!("{} => [{}] fetching...", repo_path.display(), remote_name);

	let mut cb = RemoteCallbacks::new();

	// TODO: Find a better way for this
	let mut ssh_attempts = 0;
	let mut username_attempts = 0;
	let mut username_pass_attempts = 0;

	// Took inspiration from: https://github.com/rust-lang/cargo/blob/a41c8eae701c33abd327d13ff5c057389d8801b9/src/cargo/sources/git/utils.rs#L410-L624
	cb.credentials(|url, username, cred_type| {
		if cred_type.is_ssh_key() && ssh_attempts < 2 {
			ssh_attempts = ssh_attempts + 1;
			return git2::Cred::ssh_key_from_agent(username.unwrap());
		}

		if cred_type.is_username() && username_pass_attempts < 2 {
			username_attempts = username_attempts + 1;
			return git2::Cred::username(username.unwrap());
		}

		if cred_type.is_user_pass_plaintext() && username_pass_attempts < 2 {
			username_pass_attempts = username_pass_attempts + 1;
			let cfg = git2::Config::open_default().unwrap();

			return git2::Cred::credential_helper(&cfg, url, username);
		}

		if cred_type.is_default() {
			return git2::Cred::default();
		}

		Err(git2::Error::from_str("no authentication available"))
	});

	let mut fo = FetchOptions::new();
	fo.remote_callbacks(cb);

	match remote.download(&[], Some(&mut fo)) {
		Ok(_) => {
			{
				let stats = remote.stats();
				if stats.received_bytes() > 0 {
					println!(
						"{} => [{}] received {} objects at {} bytes",
						repo_path.display(),
						remote.name()?,
						stats.total_objects(),
						stats.received_bytes()
					);
				} else {
					println!("{} => [{}] up to date", repo_path.display(), remote.name()?);
				}
			}

			remote.disconnect();

			remote
				.update_tips(None, true, git2::AutotagOption::Unspecified, None)
				.unwrap();
		}
		Err(e) => {
			eprintln!(
				"{} => failed fetching \"{}\" with message: {:?}",
				repo_path.display(),
				remote.name()?,
				e.message()
			);
		}
	}

	Some(())
}
