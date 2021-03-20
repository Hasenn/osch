use cpal::{
    Data, OutputCallbackInfo, Sample, SampleFormat, SampleRate, StreamConfig, StreamInstant,
};

pub trait Osc {
    fn set_frequency(&mut self, hertz: f32);
    fn generate<T: Sample>(&mut self, data : &mut [T]);
}

pub trait Dsp {
    fn channel_count() -> usize;
    fn process<T: Sample>(&mut self, data: &mut [T]);
}

#[derive(Clone)]
pub struct SimpleOsc {
    sample_rate: u32,
    phase: f32,
    frequency: f32,
    mod1_clock: f32,
}

impl SimpleOsc {
    pub fn new(sample_rate: u32, frequency: f32) -> SimpleOsc {
        SimpleOsc {
            sample_rate: sample_rate,
            phase: 0.,
            frequency: frequency,
            mod1_clock: 0.
        }
    }
}

impl Osc for SimpleOsc {
    fn set_frequency(&mut self, hertz: f32) {
        self.frequency = hertz;
    }
    #[inline]
    fn generate<T: Sample>(&mut self, data : &mut [T]) {
        let isr = 1.0 / (self.sample_rate as f32);
        for sample in data.iter_mut() {
            let value = sample.to_f32();
            self.phase = (self.phase + self.frequency * isr).fract();
            self.mod1_clock = (self.mod1_clock + isr).fract();
            
            let mut out = (self.phase * std::f32::consts::TAU).sin();
            out += (self.phase * 2. * std::f32::consts::TAU).sin();

            self.frequency += 2000. * isr * (self.mod1_clock * 2. * std::f32::consts::TAU).sin();
            *sample = Sample::from(&(out+value));
        }
    }
}


impl Dsp for SimpleOsc {
    fn channel_count() -> usize {1}

    fn process<T: Sample>(&mut self, data: &mut [T]) {
        self.generate(data)
    }
}