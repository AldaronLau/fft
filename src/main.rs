use pitch;
use num_complex::Complex;
use chfft::CFft1D;

use std::io::Read;
use std::convert::TryInto;

// 48 kHz (fs)
const SAMPLE_FREQUENCY: usize = 48_000;

// Power of 2 closest to 1 second (bl)
const NUM_SAMPLES: usize = 32_768;

// Nyquist frequency: maximum frequency (fn)
const BANDWIDTH: usize = SAMPLE_FREQUENCY / 2;

// Number of seconds (D)
const MEASURE_DURATION: f64 = NUM_SAMPLES as f64 / SAMPLE_FREQUENCY as f64;

// Frequency resolution - spacing between 2 measurement results (df)
const FREQUENCY_SPACING: f64 = SAMPLE_FREQUENCY as f64 / NUM_SAMPLES as f64;

fn main() {
    // Load the audio from a file.
    let mut input = vec![];
    let mut file = std::fs::File::open("Bb1-f-48kF64.raw").unwrap();
    file.read_to_end(&mut input).unwrap();
    let mut audio = vec![];
    for i in (0..input.len()).step_by(8) {
        // Hann Window
        let mult = (std::f64::consts::PI * ((i/8) as f64) / (NUM_SAMPLES as f64)).sin();
        let out = mult * mult * f64::from_le_bytes(input[i..i+8].try_into().unwrap());

        audio.push(Complex::new(out, 0.0));
    }

    let mut fft = CFft1D::<f64>::with_len(NUM_SAMPLES);
    let output = fft.forward(&audio[..NUM_SAMPLES]);

//    println!("the transform of {:?} is {:?}", audio, output);

    let fs_div_n = SAMPLE_FREQUENCY as f64 / NUM_SAMPLES as f64;

    for i in 0..=NUM_SAMPLES / 2 {
        println!("{}: {}", i as f64 * fs_div_n, (output[i].re * output[i].re + output[i].im * output[i].im).sqrt());
    }

    //
//    let (pitch, volume) = pitch::detect(&audio);

/*    println!("Fundamental: {}/{}, expect 58.27=Bb1", pitch, volume);

    for sample in audio {
        
    }*/

/*    let mut bucket = vec![0.0; BANDWIDTH];
    let mut count = vec![0usize; BANDWIDTH];
    let mut i = 0;

    for sample in audio {
        for j in 0..BANDWIDTH {
            if i % (j + 1) == 0 {
                bucket[j] += sample.abs();
                count[j] += 1;
            }
        }
        i += 1;
    }

    for i in 0..BANDWIDTH {
        bucket[i] /= (count[i] as f64 / 100.0);
        println!("{}: {}", i + 1, bucket[i]);
    }*/
}
