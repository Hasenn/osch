use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    Data, OutputCallbackInfo, Sample, SampleFormat, SampleRate, StreamConfig, StreamInstant,
};
use std::{thread, time::Duration};

#[derive(Clone)]
struct SynthState {
    sample_rate: u32,
    phase: f32,
    frequency: f32,
    mod1_clock: f32,
}
impl SynthState {
    fn new(sample_rate: u32, frequency: f32) -> SynthState {
        SynthState {
            sample_rate: sample_rate,
            phase: 0.0f32,
            frequency: frequency,
            mod1_clock: 0.,
        }
    }
    fn run<T: Sample>(&mut self, data: &mut [T], _: &cpal::OutputCallbackInfo) {
        let isr = 1.0 / (self.sample_rate as f32);
        for sample in data.iter_mut() {
            self.phase = (self.phase + self.frequency * isr).fract();
            self.mod1_clock = (self.mod1_clock + isr).fract();
            
            let mut out = (self.phase * std::f32::consts::TAU).sin();
            out += (self.phase * 2. * std::f32::consts::TAU).sin();

            self.frequency += 2000. * isr * (self.mod1_clock * 2. * std::f32::consts::TAU).sin();
            *sample = Sample::from(&out);
        }
    }
}

fn main() {
    // cpal plumbing
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let sample_format = supported_config.sample_format();
    let config: StreamConfig = supported_config.into();

    #[cfg(debug)]
    println!(
        "
Config : {:?}
Device : {:?}
        ",
        &config,
        &device.name()
    );

    let mut state = SynthState::new(config.sample_rate.0, 0.);

    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(
            &config,
            move |data, info| state.run::<f32>(data, info),
            err_fn,
        ),
        SampleFormat::I16 => device.build_output_stream(
            &config,
            move |data, info| state.run::<i16>(data, info),
            err_fn,
        ),
        SampleFormat::U16 => device.build_output_stream(
            &config,
            move |data, info| state.run::<i16>(data, info),
            err_fn,
        ),
    }
    .unwrap();
    
    stream.play().unwrap();
    thread::sleep(Duration::from_secs(5));
}
