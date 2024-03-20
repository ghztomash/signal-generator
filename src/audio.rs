use crate::app::WAVEFORMS_COUNT;
use std::sync::{Arc, Mutex};
use waveforms_rs::Waveform;
use color_eyre::eyre::{Result, OptionExt};

#[cfg(feature = "pulse")]
extern crate pulseaudio_simple_device as pulse;
#[cfg(feature = "pulse")]
use pulse::{config::Config, device::Device, stream::Stream};

#[cfg(feature = "cpal")]
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream, Device, StreamConfig
};

#[derive(Default)]
pub struct AudioStream {
    pub waveforms: Arc<Mutex<Vec<Waveform>>>,
    stream: Option<Stream>,
}

impl AudioStream {
    pub fn create_stream(&mut self) -> Result<()> {
        let device = self.create_device()?;
        let config = self.create_config(&device)?;
        let sample_rate = config.sample_rate.0 as f32;

        // Create audio thread waveforms
        let mut waveforms: Vec<Waveform> = Vec::new();
        for i in 0..WAVEFORMS_COUNT {
            waveforms.push(Waveform::new(sample_rate, 440.0 * (i as f32 + 1.0)));
        }
        self.waveforms = Arc::new(Mutex::new(waveforms));

        self.create_stream_inner(&device, &config)
    }

    fn create_device(&self) -> Result<Device> {
        self.create_device_inner()
    }

    fn create_config(&self, device: &Device) -> Result<StreamConfig> {
        self.create_config_inner(device)
    }

    fn create_stream_inner(&mut self, device: &Device, config: &StreamConfig) -> Result<()> {
        println!("Default output config : {:?}", config);
        let channels = config.channels as usize;

        // callbacks
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let thread_waveforms = Arc::clone(&self.waveforms);

        #[cfg(feature = "cpal")]
        let data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let mut waveforms = thread_waveforms.lock().unwrap();
                // copy the same value to all channels
                let value = waveforms[0].process();
                for sample in frame {
                    *sample = value;
                }
            }
        };

        self.stream = Some(device.build_output_stream(config, data_fn, err_fn, None)?);
        Ok(())
    }

    pub fn start_stream(&mut self) -> Result<()> {
        if let Some(stream) = &self.stream {
            stream.play()?;
        }
        Ok(())
    }

    pub fn stop_stream(&mut self) -> Result<()> {
        if let Some(stream) = &self.stream {
            stream.pause()?;
            self.stream = None;
        }
        Ok(())
    }
}

#[cfg(feature = "cpal")]
impl AudioStream {
    fn create_device_inner(&self) -> Result<Device> {
        let host = cpal::default_host();
        host.default_output_device().ok_or_eyre("Failed to create a default output device")
    }

    fn create_config_inner(&self, device: &Device) -> Result<StreamConfig> {
        Ok(device.default_output_config()?.config())
    }
}

#[cfg(feature = "pulse")]
impl AudioStream {
    fn create_device_inner(&self) -> Result<Device> {
        let host = cpal::default_host();
        host.default_output_device().ok_or_eyre("Failed to create a default output device")
    }

    // create stream for pulse audio
    fn create_stream_inner(&mut self) {
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
}
