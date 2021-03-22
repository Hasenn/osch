use crate::osc::{Dsp, Osc};
use cpal::Sample;

pub struct MultiOsc<U: Osc + Dsp + Sized> {
    oscs: Vec<U>,
    paused: bool,
}

impl<U: Osc + Dsp> MultiOsc<U> {
    pub fn new(oscs: Vec<U>) -> Self {
        Self {
            oscs: oscs,
            paused: false,
        }
    }
}

impl<U: Osc + Dsp> Osc for MultiOsc<U> {
    fn set_frequency(&mut self, hertz: f32) {
        for (i, osc) in self.oscs.iter_mut().enumerate() {
            osc.set_frequency(hertz * ((i + 1) as f32));
        }
    }
    #[inline]
    fn generate<S: Sample>(&mut self, data: &mut [S]) {
        for osc in self.oscs.iter_mut() {
            osc.generate(data);
        }
    }
    fn pause(&mut self, b: bool) {
        self.paused = b;
    }
}

impl<U: Osc + Dsp> Dsp for MultiOsc<U> {
    fn channel_count() -> usize {
        1
    }

    fn process<S: Sample>(&mut self, data: &mut [S]) {
        if !self.paused {
            self.generate(data)
        }
    }
}
