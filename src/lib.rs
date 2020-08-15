pub struct BeatMessage {
	pub bpm: f64,
	pub since_last: usize,
}

pub static sample_rate: usize = 44100;

pub fn detect_beats(track: Vec<i16>) -> Vec<BeatMessage> {
	let avg: f64 = track
		.iter()
		.take(sample_rate)
		.map(|x| *x as f64 / sample_rate as f64)
		.sum();
	let max = track.iter().max().cloned().unwrap_or_else(|| 0i16);
	let mut last = 0;
	track
		.iter()
		.enumerate()
		.map(|(i, x)| {
			if *x >= max {
				let lastlast = last;
				last = i;
				Some(BeatMessage {
					bpm: 60f64 / (i - lastlast) as f64 * sample_rate as f64,
					since_last: lastlast,
				})
			} else {
				None
			}
		})
		.filter_map(|x| x)
		.collect()
}
