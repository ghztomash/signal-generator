use crate::app::WAVEFORMS_COUNT;
use std::sync::{Arc, Mutex};
use waveforms_rs::Waveform;

#[cfg(feature = "pulse")]
extern crate pulseaudio_simple_device as pulse;
#[cfg(feature = "pulse")]
use pulse::{config::Config, device::Device, stream::Stream};

#[derive(Debug, Default)]
pub struct AudioStream {
    pub waveforms: Arc<Mutex<Vec<Waveform>>>,

    #[cfg(feature = "pulse")]
    stream: Stream,
}

impl AudioStream {
    pub fn create_stream(&mut self) {
        let sample_rate = 44100.0;

        // Create audio thread waveforms
        let mut waveforms: Vec<Waveform> = Vec::new();
        for _ in 0..WAVEFORMS_COUNT {
            waveforms.push(Waveform::new(sample_rate, 440.0));
        }
        self.waveforms = Arc::new(Mutex::new(waveforms));
        let thread_waveforms = Arc::clone(&self.waveforms);
    }

    #[cfg(feature = "pulse")]
    pub fn create_stream_pulse(&mut self) {
        let config = Config::default();
        let device = Device::new(env!("CARGO_PKG_NAME").to_string());

        let channels = config.channels as usize;
        let sample_rate = config.sample_rate as f32;

        // callbacks
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let data_fn = move |data: &mut [f32]| {
            for frame in data.chunks_mut(channels) {
                let mut waveforms = thread_waveforms.lock().unwrap();
                // copy the same value to all channels
                let value = waveforms[0].process();
                for sample in frame {
                    *sample = value;
                }
            }
        };

        self.stream = device.build_output_stream(&config, data_fn, err_fn).ok();
    }

    pub fn start_stream(&mut self) {
        #[cfg(feature = "pulse")]
        if let Some(stream) = &self.stream {
            stream.play().unwrap();
        }
    }

    pub fn stop_stream(&mut self) {
        #[cfg(feature = "pulse")]
        if let Some(stream) = &self.stream {
            stream.pause().unwrap();
            self.stream = None;
        }
    }
}
