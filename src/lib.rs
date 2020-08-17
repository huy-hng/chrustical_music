use std::sync::mpsc::{Receiver};

pub struct BeatMessage {
	pub bpm: f64,
	pub since_last: usize,
}

pub async fn detect_beats(samples: Receiver<i16>, sample_rate: usize) -> Vec<BeatMessage> {
	let mut max = 0usize;
	let mut last = 0usize;

	// Check for overflows in enumerate and abs
	samples
		.iter()
		.enumerate()
		.map(|(i, s)| {
		if s.abs() as usize > max {
			max = s.abs() as usize
		}
		if s.abs() as usize >= max {
			let lastlast = last;
			last = i;
			Some(BeatMessage {
				bpm: 60f64 / (i - lastlast) as f64 * sample_rate as f64,
				since_last: lastlast,
			})
		} else {
			None
		}
	}).collect::<Option<_>>().unwrap()

	// let track = (0..sample_rate).map(|| samples.recv()).collect();
	// let max = track.iter().max().cloned().unwrap_or_else(|| 0i16);
	// let mut last = 0;
	// track
	// 	.iter()
	// 	.enumerate()
	// 	.map(|(i, x)| {
	// 		if *x >= max {
	// 			let lastlast = last;
	// 			last = i;
	// 			Some(BeatMessage {
	// 				bpm: 60f64 / (i - lastlast) as f64 * sample_rate as f64,
	// 				since_last: lastlast,
	// 			})
	// 		} else {
	// 			None
	// 		}
	// 	})
	// 	.filter_map(|x| x)
	// 	.collect()
}
