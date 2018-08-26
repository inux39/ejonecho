extern crate clap;
extern crate time;

use clap::{Arg, App};

fn main() {
	let clap = App::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about("ejo says")
		.arg(Arg::with_name("time")
			.short("t")
			.help("Output about time"))
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
	if clap.is_present("time") {
		let now = time::now();
		let ejotime = ejotime(now.tm_hour, now.tm_min);
		let han = if is_half(now.tm_min) { "半" } else { "" };
		let (hour, pm) = pm_time(ejotime);
		println!(":ejoneco: < {}{}時{}", says.trim(),
			if pm {
				format!("午後{}", hour)
			} else {
				format!("午前{}", hour)
			}
			, han);
	} else {
		println!(":ejoneco: < {}", says.trim());
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

fn pm_time(t: i32) -> (i32, bool) {
	if t > 12 {
		(t - 12, true)
	} else if t == 12 {
		(12, true)
	} else if t == 0 {
		(12, false)
	} else {
		(t, false)
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

#[test]
fn test_pm_time() {
	assert!(pm_time(12) == (12, true));
	assert!(pm_time(0) == (12, false));
	assert!(pm_time(15) == (3, true));
	assert!(pm_time(1) == (1, false));
}

