/* This example expose parameter to pass generator of sample.
Good starting point for integration of cpal into your application.
*/

extern crate anyhow;
extern crate clap;
extern crate cpal;

use cpal::{FromSample, Sample};
use cpal::{
    I24, SizedSample, U24,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

mod notes;

const TOTAL_TIME_MILLIS: u64 = 5000;

fn main() -> anyhow::Result<()> {
    let mel = notes::Melody::from_list_of_tuples(vec![
        (0, 225.0),
        (10, 0.0),
        (12, 225.0),
        (15, 450.0),
        (20, 0.0),
        (30, 300.0),
        (32, 323.0),
        (34, 300.0),
        (38, 0.0),
    ]);
    let stream = stream_setup_for(mel)?;
    stream.play()?;
    std::thread::sleep(std::time::Duration::from_millis(TOTAL_TIME_MILLIS));
    println!("stopped");
    Ok(())
}

pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
    WhatDis,
}

pub struct Oscillator {
    pub sample_rate: f32,
    pub waveform: Waveform,
    pub current_sample_index: f32,
    pub frequency_hz: f32,
}

impl Oscillator {
    fn advance_sample(&mut self) {
        self.current_sample_index = (self.current_sample_index + 1.0) % self.sample_rate;
    }

    fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    fn set_frequency(&mut self, freq: f32) {
        // add bounds for human hearing range
        // TODO: add functionality for real scales
        self.frequency_hz = freq;
    }

    fn calculate_sine_output_from_freq(&self, freq: f32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        (self.current_sample_index * freq * two_pi / self.sample_rate).sin()
    }

    fn is_multiple_of_freq_above_nyquist(&self, multiple: f32) -> bool {
        self.frequency_hz * multiple > self.sample_rate / 2.0
    }

    fn sine_wave(&mut self) -> f32 {
        self.advance_sample();
        self.calculate_sine_output_from_freq(self.frequency_hz)
    }

    fn generative_waveform(&mut self, harmonic_index_increment: i32, gain_exponent: f32) -> f32 {
        self.advance_sample();
        let mut output = 0.0;
        let mut i = 1.0;
        while !self.is_multiple_of_freq_above_nyquist(i) {
            let gain = 1.0 / i.powf(gain_exponent);
            output += gain * self.calculate_sine_output_from_freq(self.frequency_hz * i);
            i += harmonic_index_increment as f32;
        }
        output
    }

    fn square_wave(&mut self) -> f32 {
        self.generative_waveform(2, 1.0)
    }

    fn saw_wave(&mut self) -> f32 {
        self.generative_waveform(1, 1.0)
    }

    fn triangle_wave(&mut self) -> f32 {
        self.generative_waveform(2, 2.0)
    }

    fn what_dis_wave(&mut self) -> f32 {
        self.generative_waveform(4, 1.0)
    }

    fn tick(&mut self) -> f32 {
        match self.waveform {
            Waveform::Sine => self.sine_wave(),
            Waveform::Square => self.square_wave(),
            Waveform::Saw => self.saw_wave(),
            Waveform::Triangle => self.triangle_wave(),
            Waveform::WhatDis => self.what_dis_wave(),
        }
    }
}

pub fn stream_setup_for(mel: notes::Melody) -> Result<cpal::Stream, anyhow::Error>
where
{
    let (_host, device, config) = host_device_setup()?;

    match config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(&device, &config.into(), mel),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, &config.into(), mel),
        cpal::SampleFormat::I24 => make_stream::<I24>(&device, &config.into(), mel),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, &config.into(), mel),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, &config.into(), mel),
        cpal::SampleFormat::U8 => make_stream::<u8>(&device, &config.into(), mel),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, &config.into(), mel),
        // cpal::SampleFormat::U24 => make_stream::<U24>(&device, &config.into()),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, &config.into(), mel),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, &config.into(), mel),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, &config.into(), mel),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, &config.into(), mel),
        sample_format => Err(anyhow::Error::msg(format!(
            "Unsupported sample format '{sample_format}'"
        ))),
    }
}

pub fn host_device_setup()
-> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
    println!("Output device : {}", device.name()?);

    let config = device.default_output_config()?;
    println!("Default output config : {config:?}");

    Ok((host, device, config))
}

fn wave_round(time_diff: f32, time_per_round: f32) -> Waveform {
    let mut diff_in_cycle = time_diff.clone();
    let cycle_length = 5.0 * time_per_round;
    while diff_in_cycle > cycle_length {
        diff_in_cycle -= cycle_length;
    }
    match (diff_in_cycle / time_per_round).floor() as i32 {
        0 => Waveform::Sine,
        1 => Waveform::Saw,
        2 => Waveform::Square,
        3 => Waveform::Triangle,
        4 => Waveform::WhatDis,
        _ => Waveform::Sine,
    }
}

fn melody_at_time(melody_mapping: &Vec<f32>, time_diff: f32, subdiv_size: f32) -> f32 {
    // TODO: calc subdiv index from time_diff and subdiv size
    let subdiv_idx = (time_diff / subdiv_size).floor() as usize;
    let modded_idx = subdiv_idx % melody_mapping.len();
    melody_mapping[modded_idx]
}

pub fn make_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    melody: notes::Melody,
) -> Result<cpal::Stream, anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let num_channels = config.channels as usize;
    let mut oscillator = Oscillator {
        waveform: Waveform::Sine,
        sample_rate: config.sample_rate.0 as f32,
        current_sample_index: 0.0,
        frequency_hz: 440.0,
    };
    let err_fn = |err| eprintln!("Error building output sound stream: {err}");

    let time_at_start = std::time::Instant::now();
    println!("Time at start: {time_at_start:?}");

    let subdiv_size = 0.1;
    let length_in_subdivisions: usize = 40;

    let melody_mapping = melody.mapping(length_in_subdivisions);
    println!("melody mapping: {:?}", melody_mapping);
    let mut cur_freq = 0.0;
    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            // for 0-1s play sine, 1-2s play square, 2-3s play saw, 3-4s play triangle_wave
            let time_since_start = std::time::Instant::now()
                .duration_since(time_at_start)
                .as_secs_f32();
            let waveform = wave_round(time_since_start, 0.2);
            oscillator.set_waveform(waveform);
            let freq = melody_at_time(&melody_mapping, time_since_start, subdiv_size);
            if freq != cur_freq {
                oscillator.set_frequency(freq);
                cur_freq = freq;
                println!("curr freq: {}", cur_freq);
            }
            process_frame(output, &mut oscillator, num_channels)
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}

fn process_frame<SampleType>(
    output: &mut [SampleType],
    oscillator: &mut Oscillator,
    num_channels: usize,
) where
    SampleType: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(num_channels) {
        let value: SampleType = SampleType::from_sample(oscillator.tick());

        // copy the same value to all channels
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
