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
    println!("Hello, world!");
}
