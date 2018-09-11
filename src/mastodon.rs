use super::std as std;
use super::toml as toml;
use super::mammut as mammut;
use mammut::{Data, Mastodon, Registration};
use mammut::apps::{AppBuilder, Scopes};
use mammut::status_builder::StatusBuilder;
use std::fs::File;
use std::io::{Read, Write};

const FILE: &'static str = "mastodon.toml";

fn register() -> Mastodon{
	let app = AppBuilder {
		client_name: "ejonecho",
		redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
		scopes: Scopes::Write,
		website: Some("https://github.com/inux39/ejonecho"),
	};
	let base = {
		let mut s = String::new();
		println!("Instance URL:");
		std::io::stdin().read_line(&mut s).unwrap();
		if !s.starts_with("https") {
			s = format!("https://{}", s);
		}
		s.trim().to_string()
	};

	let mut registration = Registration::new(base);
	registration.register(app).unwrap();
	let url = registration.authorise().unwrap();

	println!("Authorize URL: {}", url);
	println!("returned code: ");

	let code = {
		let mut s = String::new();
		std::io::stdin().read_line(&mut s).unwrap();
		s.trim().to_string()
	};

	let mastodon = registration.create_access_token(code.to_string()).unwrap();

	let toml = toml::to_string(&*mastodon).unwrap();
	let mut file = File::create(FILE).unwrap();
	file.write_all(toml.as_bytes()).unwrap();

	mastodon
}

pub fn toot(t: String) {
	let mastodon = match File::open(FILE) {
		Ok(mut file) => {
			let mut config = String::new();
			file.read_to_string(&mut config).unwrap();
			let data: Data = toml::from_str(&config).unwrap();
			Mastodon::from_data(data)
		},
		Err(_) => register(),
	};
	let mut status = StatusBuilder::new(t);
	status.visibility = Some(mammut::status_builder::Visibility::Unlisted);
	mastodon.new_status(status).unwrap();
}

