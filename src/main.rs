mod error;
mod mastodon;

use clap::{Arg, App};
use std::error::Error;

fn main() {
	let clap = App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.version_short("v")
		.author(env!("CARGO_PKG_AUTHORS"))
		.about("ejo says")
		.arg(Arg::with_name("time")
			.short("t")
			.help("Output about time"))
		.arg(Arg::with_name("ampm")
			.short("s")
			.help("Output a.m./p.m. time"))
		.arg(Arg::with_name("post")
			.long("post")
			.help("Post to Mastodon"))
		.arg(Arg::with_name("INPUT")
			.takes_value(true)
			.help("A word to speak"))
		.get_matches();

	let says = match clap.value_of_lossy("INPUT") {
		Some(s) => s,
		None => {
			let mut s = String::new();
			std::io::stdin().read_line(&mut s).unwrap();
			std::borrow::Cow::from(s.clone())
		},
	};
	let ejo_says = format!(":ejoneco: < {}{}", says.trim(),
		if clap.is_present("time") {
			ejo_jiho(clap.is_present("ampm"))
		} else {
			"".to_string()
		}
	);

	if clap.is_present("post") {
		println!("{}", ejo_says);
		match mastodon::toot(ejo_says) {
			Ok(_) => {},
			Err(e) => println!("{}", e.description()),
		};
	} else {
		println!("{}", ejo_says);
	}
}

fn ejo_jiho(ampm: bool) -> String {
	let now = time::now();
	let ejotime = ejotime(now.tm_hour, now.tm_min);
	let han = is_half_to_str(now.tm_min);
	let hour = pm_time(ejotime);

	format!("{}時{}",
		if ampm {
			hour
		} else {
			ejotime
		},
		han
	)
}

fn is_half_to_str(min: i32) -> &'static str {
	if is_half(min) {
		"半"
	} else {
		""
	}
}

fn is_half(m: i32) -> bool {
	if m >= 20 && m <= 40 {
		true
	} else {
		false
	}
}

fn ejotime(h: i32, m: i32) -> i32 {
	if m + 20 > 60 {
		h + 1
	} else {
		h
	}
}

fn pm_time(t: i32) -> i32 {
	if t > 12 {
		t - 12
	} else if t == 12 {
		12
	} else if t == 0 {
		12
	} else {
		t
	}
}

#[test]
fn test_ejotime() {
	assert!(ejoneco_time(0, 0) == 0);
	assert!(ejoneco_time(0, 40) == 0);
	assert!(ejoneco_time(0, 41) == 1);
	assert!(ejoneco_time(12, 50) == 13);
	assert!(ejoneco_time(24, 55) == 25);
}

#[test]
fn test_is_half() {
	assert!(is_half(19) == false);
	assert!(is_half(20) == true);
	assert!(is_half(30) == true);
	assert!(is_half(40) == true);
	assert!(is_half(41) == false);
}

/*
#[test]
fn test_pm_time() {
	assert!(pm_time(12) == (12, true));
	assert!(pm_time(0) == (12, false));
	assert!(pm_time(15) == (3, true));
	assert!(pm_time(1) == (1, false));
}
*/

