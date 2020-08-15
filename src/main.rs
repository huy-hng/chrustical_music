use cpal::traits::HostTrait;

fn main() {
	let host = cpal::default_host();
	let device = host
		.default_input_device()
		.expect("no output device available");
}
