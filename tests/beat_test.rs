use chrustical_music::detect_beats;
use hound;

static KICK: &[u8; 211068] = include_bytes!("./tictacshutup/428__tictacshutup__prac-kick.wav");

#[test]
fn concat_kick() {
	let reader = hound::WavReader::new(KICK as &[u8]).unwrap();
	let kick: Vec<i16> = reader
		.into_samples::<i16>()
		.filter_map(Result::ok)
		.collect();
	let kick_length = kick.len();
	let track: Vec<i16> = kick.into_iter().cycle().take(kick_length * 4).collect();

	match detect_beats(track).last() {
		Some(s) => assert_eq!(s.bpm.round(), 25f64.round()),
		None => panic!("No BeatMessage"),
	}
}
