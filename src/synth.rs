use std::sync::atomic::{AtomicBool, Ordering};

use cpal::Sample;

use crate::sync::{AtomicF32, Sharer};

pub trait Dsp {
    fn channel_count(&self) -> usize {
        1
    }
    fn process<T: Sample>(&self, data: &mut [T]);
}
pub trait SynthInterface {
    fn set_frequency(&self, hertz: f32);
    fn trigger(&self, on: bool);
}

pub struct MySynthShared {
    frequency: AtomicF32,
    playing: AtomicBool,
}

impl MySynthShared {
    pub fn new(frequency: f32, playing: bool) -> Self {
        MySynthShared {
            frequency: AtomicF32::new(frequency),
            playing: AtomicBool::new(playing),
        }
    }
}

pub struct MySynthOwned {
    phase: f32,
    sample_rate: f32,
    envtime: f32
}

impl MySynthOwned {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            phase: 0.,
            sample_rate,
            envtime: 0.
        }
    }
}

impl SynthInterface for MySynthShared {
    fn set_frequency(&self, hertz: f32) {
        self.frequency.store(hertz, Ordering::Relaxed)
    }

    fn trigger(&self, on: bool) {
        self.playing.store(on, Ordering::Relaxed)
    }
}

pub type MySynth = Sharer<MySynthShared, MySynthOwned>;

impl MySynth {
    pub fn init(sample_rate: f32, frequency: f32) -> Self {
        Sharer::new(
            MySynthShared::new(frequency, false),
            MySynthOwned::new(sample_rate),
        )
    }
}

impl Dsp for MySynth {
    fn process<T: Sample>(&self, data: &mut [T]) {
        let mut owned = self.owned.borrow_mut();
        let trigger = self.shared.playing.swap( false, Ordering::Relaxed);
        if trigger {
            owned.envtime = 0.;
        }
        let isr = 1.0 / (owned.sample_rate as f32);
        let frequency = self.shared.frequency.load(Ordering::Relaxed);
        
        for sample in data.iter_mut() {
            let value = sample.to_f32();
            owned.phase = (owned.phase + frequency * isr).fract();
            owned.envtime = (owned.envtime + 5.* isr).clamp(0., 1.);
            let mut out = 
                   0.8 * (owned.phase * 1. * std::f32::consts::TAU).sin();
            out += 0.3 * (owned.phase * 2. * std::f32::consts::TAU).sin();
            out += 0.2 * (owned.phase * 3. * std::f32::consts::TAU).sin();
            out += 0.1 * (owned.phase * 5. * std::f32::consts::TAU).sin();
            out *= owned.envtime;

            *sample = Sample::from(&(out + value));
        }
    }

    fn channel_count(&self) -> usize {
        1
    }
}
