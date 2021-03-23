extern crate osch;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, StreamConfig};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

use osch::{
    note,
    synth::{Dsp, MySynth, SynthInterface},
};

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

    println!(
        "
Config : {:?}
Device : {:?}
Sample Format : {:?}
        ",
        &config,
        &device.name(),
        sample_format
    );

    let _synth = MySynth::init(config.sample_rate.0 as f32, 444.);
    let synth = _synth.get_shared();

    let stream = match sample_format {
        SampleFormat::I16 => device.build_output_stream(
            &config,
            move |data, _| {
                for sample in data.iter_mut() {
                    *sample = Sample::from(&0.);
                }
                _synth.process::<i16>(data);
            },
            err_fn,
        ),
        _ => unimplemented!(),
    }
    .unwrap();

    stream.play().unwrap();
    
    let note_dur = 60u64;

    for i in 0..(127-7) {
        // root
        &synth.set_frequency(note::midi(i));
        &synth.trigger(true);
        thread::sleep(Duration::from_millis(note_dur));
        &synth.trigger(false);

        // minor third
        &synth.set_frequency(note::midi(
            (i + 3) % 128
        ));
        &synth.trigger(true);
        thread::sleep(Duration::from_millis(note_dur));
        &synth.trigger(false);

        // perfect fifth
        &synth.set_frequency(note::midi(
            (i + 7) % 128
        ));
        &synth.trigger(true);
        thread::sleep(Duration::from_millis(note_dur));
        &synth.trigger(false);
    }



}