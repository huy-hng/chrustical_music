use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
	let host = cpal::default_host();
	let device = host
		.default_input_device()
		.expect("Failed to get default input device");
	println!("Default input device: {}", device.name().expect("no name"));
}
