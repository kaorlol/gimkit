use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_random_number(range: std::ops::Range<u64>) -> u64 {
	let start = SystemTime::now();
	let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
	let random_seed = since_the_epoch.as_secs() as u64;

	let mut rng = (random_seed % (range.end - range.start)) + range.start;

	if rng < range.start {
		rng += range.start;
	} else if rng >= range.end {
		rng -= range.start;
	}

	rng
}