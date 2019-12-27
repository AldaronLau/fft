extern crate twang; // for sound generation / effects
// extern crate adi; // for speaker

use std::io::Write;

// use adi::speaker::Speaker;
use twang::{Sound, prelude::*};

use opus_no::StreamEncoder;

fn main() {
	// let mut speaker = Speaker::new(0, false).unwrap();
	let trombone = include!("spectral.rs");

    println!("{}", trombone.len());

	let mut gen = Sound::new(None, 1.0); // A3

/*	loop {
		speaker.update(&mut || {
			(gen.next().unwrap().ovr(&trombone)).into()
		});
	}*/

    let mut opus_stream = StreamEncoder::new();
    let mut opus_file = vec![];
    let mut audio_buffer = vec![];

    let mut buffer = vec![];

//    while let Some((head, body)) = opus_stream.page() {
    opus_file.extend(opus_stream.head());
//        opus_file.extend(body);
//    }

    for _ in 0..48_000 {
        let sample: i16 = (gen.next().unwrap().ovr(&trombone)).into();

        let [a, b] = sample.to_le_bytes();
        buffer.push(a);
        buffer.push(b);

        audio_buffer.push(sample);
        audio_buffer.push(sample);

        if audio_buffer.len() >= 1920 * 2 {
            let mut buff = [0; 1920 * 2];
            for i in 0..1920 * 2 {
                buff[i] = audio_buffer[i];
            }
            audio_buffer.clear();
            opus_stream.encode(&buff);
            while let Some((head, body)) = opus_stream.page() {
                opus_file.extend(head);
                opus_file.extend(body);
            }
        }
    }

    let mut file = std::fs::File::create("out.opus").unwrap();
    file.write_all(&opus_file).unwrap();

    let mut file = std::fs::File::create("out.raw").unwrap();
    file.write_all(&buffer).unwrap();
}
