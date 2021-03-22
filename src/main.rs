mod multiosc;
mod note;
mod osc;
mod synth;
mod utils;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    Sample, SampleFormat, StreamConfig,
};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

use osc::{Dsp, Osc, SimpleOsc};

use multiosc::MultiOsc;

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

    // todo: use a ringbuffer for threading communication
    let synth = Arc::new(Mutex::new(MultiOsc::new(vec![
        SimpleOsc::new(config.sample_rate.0, 444.),
        SimpleOsc::new(config.sample_rate.0, 444.),
        SimpleOsc::new(config.sample_rate.0, 444.),
        SimpleOsc::new(config.sample_rate.0, 444.),
        SimpleOsc::new(config.sample_rate.0, 444.),
    ])));

    let _synth = Arc::clone(&synth);

    let stream = match sample_format {
        SampleFormat::I16 => device.build_output_stream(
            &config,
            move |data, _| {
                for sample in data.iter_mut() {
                    *sample = Sample::from(&0.);
                }
                //osc1.process::<i16>(data);
                //osc2.process::<i16>(data);
                {
                    &_synth.lock().unwrap().process::<i16>(data);
                }
            },
            err_fn,
        ),
        _ => unimplemented!(),
    }
    .unwrap();

    stream.play().unwrap();

    let note_dur = 24u64;
    for i in 0..(127 - 7) {
        // the locking makes the timing vary..
        // we'll need to find a better solution
        synth.lock().unwrap().set_frequency(note::midi(i));
        synth.lock().unwrap().pause(false);
        thread::sleep(Duration::from_millis(note_dur));
        synth.lock().unwrap().pause(true);

        synth
            .lock()
            .unwrap()
            .set_frequency(note::midi((i + 7) % 127));
        synth.lock().unwrap().pause(false);
        thread::sleep(Duration::from_millis(note_dur));
        synth.lock().unwrap().pause(true);

        synth
            .lock()
            .unwrap()
            .set_frequency(note::midi((i + 3) % 127));
        synth.lock().unwrap().pause(false);
        thread::sleep(Duration::from_millis(note_dur));
        synth.lock().unwrap().pause(true);

        synth
            .lock()
            .unwrap()
            .set_frequency(note::midi(58 + ((i + 3) % 12)));
        synth.lock().unwrap().pause(false);
        thread::sleep(Duration::from_millis(note_dur));
        synth.lock().unwrap().pause(true);
    }
}
