use mammut::{Data, Mastodon, Registration};
use mammut::apps::{AppBuilder, Scopes};
use mammut::status_builder::StatusBuilder;
use std::fs::File;
use std::io::{Read, Write};
use super::error::Result;

const FILE: &'static str = "mastodon.toml";

fn register() -> Result<Mastodon> {
	let app = AppBuilder {
		client_name: "ejonecho",
		redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
		scopes: Scopes::Write,
		website: Some("https://github.com/inux39/ejonecho"),
	};
	let base = {
		let mut s = String::new();
		println!("Instance URL:");
		std::io::stdin().read_line(&mut s)?;
		if !s.starts_with("https") {
			s = format!("https://{}", s);
		}
		s.trim().to_string()
	};

	let mut registration = Registration::new(base);
	registration.register(app)?;
	let url = registration.authorise()?;

	println!("Authorize URL: {}", url);
	println!("returned code: ");

	let code = {
		let mut s = String::new();
		std::io::stdin().read_line(&mut s)?;
		s.trim().to_string()
	};

	let mastodon = registration.create_access_token(code.to_string())?;

	let toml = toml::to_string(&*mastodon)?;
	let file_path = config_path(FILE);
	let mut file = File::create(file_path)?;
	file.write_all(toml.as_bytes())?;

	Ok(mastodon)
}

fn config_path(filename: &'static str) -> std::path::PathBuf {
	let mut path = match std::env::current_exe() {
		Ok(p) => p,
		Err(_) => std::path::PathBuf::new(),
	};
	path.pop();
	path.push(filename);
	path
}

pub fn toot(t: String) -> Result<()> {
	let file_path = config_path(FILE);
	let mastodon = match File::open(file_path) {
		Ok(mut file) => {
			let mut config = String::new();
			file.read_to_string(&mut config)?;
			let data: Data = toml::from_str(&config)?;
			Mastodon::from_data(data)
		},
		Err(_) => register()?,
	};
	let mut status = StatusBuilder::new(t);
	status.visibility = Some(mammut::status_builder::Visibility::Unlisted);
	mastodon.new_status(status)?;
	Ok(())
}

