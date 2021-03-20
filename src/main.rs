mod osc;


use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    Data, OutputCallbackInfo, Sample, SampleFormat, SampleRate, StreamConfig, StreamInstant,
};
use std::{thread, time::Duration};

use std::sync::{
    Arc,
    Mutex
};
use osc::{
    Dsp,
    Osc
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

    let mut osc1 = osc::SimpleOsc::new(config.sample_rate.0, 220.);
    let mut osc2 = osc::SimpleOsc::new(config.sample_rate.0, 320.);
    let mut osc3 = Arc::new(Mutex::new(osc1.clone()));

    let _osc3 = Arc::clone(&osc3);
    let stream = match sample_format {
        SampleFormat::I16 => device.build_output_stream(
            &config,
            move |data, info| {
                
                for sample in data.iter_mut() {
                    *sample = Sample::from(&0.);
                }
                //osc1.process::<i16>(data);
                //osc2.process::<i16>(data);
                {
                    &_osc3.lock().unwrap().process::<i16>(data);
                }
            },
            err_fn,
        ),
        _ => unimplemented!()
    }
    .unwrap();
    
    stream.play().unwrap();
    
    thread::sleep(Duration::from_secs(1));
    osc3.lock().unwrap().set_frequency(444.);
    thread::sleep(Duration::from_secs(1));
    osc3.lock().unwrap().set_frequency(666.);
    thread::sleep(Duration::from_secs(1));

}
