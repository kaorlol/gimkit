use std::{
    ops::Range,
    time::{SystemTime, UNIX_EPOCH},
};

use regex::Regex;

pub fn get_random_number(range: Range<u64>) -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let random_seed = since_the_epoch.as_secs() as u64;
    let mut rng = (random_seed % (range.end - range.start)) + range.start;

    if rng < range.start {
        rng += range.start;
    } else if rng >= range.end {
        rng -= range.start;
    }

    rng
}

pub fn is_gimkit_link(link: &str) -> bool {
    let pattern = r"https://www.gimkit.com/join/[a-zA-Z0-9]+";
    let regex = Regex::new(pattern).unwrap();
    regex.is_match(link)
}