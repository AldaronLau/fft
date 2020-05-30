use std::io::Write;

// for sound generation / effects
use twang::{prelude::*, Sound};

// for Opus stream/file output
use opus_no::StreamEncoder;

// for web
use cala_web;

use std::collections::HashMap;
use std::future::Future;

fn request(stream: cala_web::Stream) -> Box<dyn Future<Output = ()> + Send> {
    Box::new(async {
        let mut stream = stream;

        let trombone = include!("spectral.rs");

        println!("{}", trombone.len());

        let mut gen = Sound::new(None, 1.0); // FIXME: Pass in Hz rather than multiplier

        let mut opus_stream = StreamEncoder::new();
        let mut opus_file = vec![];
        let mut audio_buffer = vec![];

        let mut buffer = vec![];

        opus_file.extend(opus_stream.head());

        for second in 0..10 {
            for _ in 0..48_000 * 6 {
                let sample: i16 = (gen.next().unwrap().ovr(&trombone)).into();

                let [a, b] = sample.to_le_bytes();
                buffer.push(a);
                buffer.push(b);

                audio_buffer.push(sample);
                audio_buffer.push(sample);

                // FIXME: Change opus-no to allow other numbers of samples for end of file.
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

            stream.push_u8(&opus_file);
            stream.send().await.unwrap();
            opus_file.clear();

            std::thread::sleep(std::time::Duration::from_millis(6_000));
        }
    })
}

fn main() {
    let mut map = HashMap::<&str, (&str, cala_web::ResourceGenerator)>::new();
    map.insert("/listen", ("application/ogg", request));

    cala_web::start("examples/serve", map);
}
