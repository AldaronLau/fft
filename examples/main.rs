extern crate twang; // for sound generation / effects
extern crate adi; // for speaker

use adi::speaker::Speaker;
use twang::{Sound, prelude::*};

fn main() {
	let mut speaker = Speaker::new(0, false).unwrap();
	let trombone = include!("spectral.rs");
	let mut gen = Sound::new(None, 1.0); // A3

	loop {
		speaker.update(&mut || {
			(gen.next().unwrap().ovr(&trombone)).into()
		});
	}
}
