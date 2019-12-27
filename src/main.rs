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
    let mut raw_audio = vec![];
    for i in (0..input.len()).step_by(8) {
        let raw_sample = f64::from_le_bytes(input[i..i+8].try_into().unwrap());
        raw_audio.push(raw_sample);

        // Hann Window
        let mult = (std::f64::consts::PI * ((i/8) as f64) / (NUM_SAMPLES as f64)).sin();
        let out = mult * mult * raw_sample;

        audio.push(Complex::new(out, 0.0));
    }

    let (pitch, vol) = pitch::detect(&raw_audio[..NUM_SAMPLES]);
    // println!("{}; {}", pitch, vol);

    let mut fft = CFft1D::<f64>::with_len(NUM_SAMPLES);
    let output = fft.forward(&audio[..NUM_SAMPLES]);

    let fs_div_n = SAMPLE_FREQUENCY as f64 / NUM_SAMPLES as f64;

    let mut list = vec![];

    for i in 0..=NUM_SAMPLES / 2 {
        let val = (i as f64 * fs_div_n, (output[i].re * output[i].re + output[i].im * output[i].im).sqrt());

        list.push(val);

        // println!("{}: {}", val.0, val.1);
    }

    // println!("=====");

    // Find start location.
//    let mut start_hz = pitch / 2.0;
    let mut end_hz = pitch / 2.0;
    let mut start = false;
    let mut max = 0.0;
    let mut current_hz = 0.0;
    let mut output = vec![];

    for (hz, amp) in list {
        if amp >= max {
            current_hz = hz;
            max = amp;
        }

        if hz >= end_hz {
            if start && max > 1.0 {
                // println!("{}: {}", current_hz, max);
                output.push((current_hz, max));
            }
            start = true;
            end_hz += pitch;
            max = 0.0;
        }
    }

    println!("{:?}", output);
}
