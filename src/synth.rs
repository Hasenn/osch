use std::sync::atomic::{AtomicBool, Ordering};

use cpal::Sample;

use crate::sync::{AtomicF32, Sharer};

pub trait Dsp {
    fn channel_count(&self) -> usize {
        1
    }
    fn process<T: Sample>(&self, data: &mut [T]);
}

/// trait for a controllable synth
/// Able to play simple melodies
pub trait Synth {
    /// Set the base frequency of the synth
    fn set_frequency(&self, hertz: f32);
    /// Triggers on or off the synth
    fn trigger(&self, on: bool);
}

pub type MySynth = Sharer<MySynthShared, MySynthOwned>;
pub struct MySynthOwned {
    phase: f32,
    sample_rate: f32,
    envelope_time: f32, // Hacky envelope
    release_time: f32,  // Hacky envelope
}
pub struct MySynthShared {
    frequency: AtomicF32,
    playing: AtomicBool,
}

const ATTACK : f32 = 5./*secs-1*/;
const RELEASE: f32 = 2./*secs-1*/;

impl Dsp for MySynth {
    fn process<T: Sample>(&self, data: &mut [T]) {
        let mut owned = self.owned.borrow_mut();
        // inverse sample rate, i.e. time delta between samples
        let isr = 1.0 / (owned.sample_rate as f32);

        // load our shared atomic values to be used for this buffer
        let playing = self.shared.playing.load(Ordering::Relaxed);
        let frequency = self.shared.frequency.load(Ordering::Relaxed);

        for sample in data.iter_mut() {
            let value = sample.to_f32();

            owned.phase = (owned.phase + frequency * isr).fract();

            if playing {
                // envelope goes up to 1 in 1/ATTACK seconds
                owned.envelope_time = (owned.envelope_time + isr * ATTACK).clamp(0., 1.);
                // reset release to 1 rapidly
                owned.release_time = (owned.release_time + isr * 40.).clamp(0., 1.)
            }
            if !playing { 
                // reset envelope rapidly
                owned.envelope_time = (owned.envelope_time - isr * 40.).clamp(0., 1.);
                // release goes down to 0 in 1/RELEASE seconds
                owned.release_time = (owned.release_time - isr * RELEASE).clamp(0., 1.);
            }

            // hardcoded additive coefficients. todo : fast_sin, and other waveforms ?
            let mut out = 0.8 * (owned.phase * 1. * std::f32::consts::TAU).sin();
            out += 0.3 * (owned.phase * 4. * std::f32::consts::TAU).sin();
            out += 0.2 * (owned.phase * 8. * std::f32::consts::TAU).sin();

            out *= owned.envelope_time;
            out *= owned.release_time;

            *sample = Sample::from(&(out + value));
        }
    }

    fn channel_count(&self) -> usize {
        1
    }
}

impl MySynthShared {
    pub fn new(frequency: f32, playing: bool) -> Self {
        MySynthShared {
            frequency: AtomicF32::new(frequency),
            playing: AtomicBool::new(playing),
        }
    }
}
impl MySynthOwned {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            phase: 0.,
            sample_rate,
            envelope_time: 0.,
            release_time: 1.,
        }
    }
}
impl MySynth {
    pub fn init(sample_rate: f32, frequency: f32) -> Self {
        Sharer::new(
            MySynthShared::new(frequency, false),
            MySynthOwned::new(sample_rate),
        )
    }
}

impl Synth for MySynthShared {
    fn set_frequency(&self, hertz: f32) {
        self.frequency.store(hertz, Ordering::Relaxed)
    }

    fn trigger(&self, on: bool) {
        self.playing.store(on, Ordering::Relaxed);
    }
}
