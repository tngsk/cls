use clap::Parser;
use std::path::PathBuf;
use thiserror::Error;

mod envelope;
mod oscillator;
mod tone;
use envelope::Envelope;
use tone::{ToneError, ToneParams};

// Command line interface for the tone generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input parameters in S-expression format
    #[arg(long)]
    params: Option<String>,

    /// Output file path
    #[arg(short, long, default_value = "output.wav")]
    output: PathBuf,

    /// Waveform type (sin, square, triangle, saw, pulse, noise)
    #[arg(short, long)]
    waveform: Option<String>,

    /// Frequency in Hz (20-20000)
    #[arg(short, long)]
    frequency: Option<f32>,

    /// Duration in seconds
    #[arg(long)]
    dur: Option<f32>,

    /// Attack time in seconds
    #[arg(short, long)]
    attack: Option<f32>,

    /// Decay time in seconds
    #[arg(short, long)]
    decay: Option<f32>,

    /// Sustain level (0.0-1.0)
    #[arg(short, long)]
    sustain: Option<f32>,

    /// Release time in seconds
    #[arg(short, long)]
    release: Option<f32>,

    /// Sample rate in Hz
    #[arg(long, default_value = "48000")]
    sample_rate: u32,

    /// Bits per sample
    #[arg(long, default_value = "32")]
    bits_per_sample: u16,
}

impl Cli {
    // Convert command line arguments to tone parameters
    fn to_tone_params(&self) -> ToneParams {
        let mut params = ToneParams::default();

        if let Some(waveform) = &self.waveform {
            params.waveform = waveform.clone();
        }
        if let Some(freq) = self.frequency {
            params.freq = freq;
        }
        if let Some(dur) = self.dur {
            params.dur = dur;
        }
        if let Some(attack) = self.attack {
            params.attack = attack;
        }
        if let Some(decay) = self.decay {
            params.decay = decay;
        }
        if let Some(sustain) = self.sustain {
            params.sustain = sustain;
        }
        if let Some(release) = self.release {
            params.release = release;
        }

        params
    }
}

// Parse S-expression parameters into ToneParams
fn parse_params(params: &str) -> Result<ToneParams, ToneError> {
    serde_lexpr::from_str::<ToneParams>(params).map_err(|e| ToneError::Parse(e.to_string()))
}

// Generate audio samples and write to WAV file

#[derive(Error, Debug)]
pub enum WavWriteError {
    #[error("Invalid bits per sample: {0}")]
    InvalidBitsPerSample(u16),

    #[error("WAV write error: {0}")]
    WriterError(#[from] hound::Error),
}

pub fn generate_and_write_wav(
    params: &ToneParams,
    path: &PathBuf,
    sample_rate: u32,
    bits_per_sample: u16,
) -> Result<(), WavWriteError> {
    let duration = params.dur;
    let num_samples = (duration * sample_rate as f32) as usize;

    let mut oscillator = oscillator::Oscillator::from_str(&params.waveform, sample_rate as f32)
        .expect("Invalid waveform type");
    oscillator.set_frequency(params.freq);
    let envelope = Envelope::new(params);

    match bits_per_sample {
        32 => {
            let spec = hound::WavSpec {
                channels: 2,
                sample_rate,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };
            let mut writer = hound::WavWriter::create(path, spec)?;
            for i in 0..num_samples {
                let time = i as f32 / sample_rate as f32;
                let raw_sample = oscillator.generate();
                let amplitude = envelope.get_amplitude(time, duration);
                let sample = raw_sample * amplitude;

                writer.write_sample(sample)?;
                writer.write_sample(sample)?; // duplicate for stereo
            }
            writer.finalize()?;
        }
        24 => {
            let spec = hound::WavSpec {
                channels: 2,
                sample_rate,
                bits_per_sample: 24,
                sample_format: hound::SampleFormat::Int,
            };
            let mut writer = hound::WavWriter::create(path, spec)?;
            for i in 0..num_samples {
                let time = i as f32 / sample_rate as f32;
                let raw_sample = oscillator.generate();
                let amplitude = envelope.get_amplitude(time, duration);
                let sample = raw_sample * amplitude;

                let int_sample = (sample * 8388607.0) as i32;
                writer.write_sample(int_sample)?;
                writer.write_sample(int_sample)?; // duplicate for stereo
            }
            writer.finalize()?;
        }
        _ => return Err(WavWriteError::InvalidBitsPerSample(bits_per_sample)),
    }
    Ok(())
}

// Main function: parse arguments, generate and save audio
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let params = if let Some(params_str) = cli.params {
        parse_params(&params_str)?
    } else {
        cli.to_tone_params()
    };

    params.validate()?;

    generate_and_write_wav(&params, &cli.output, cli.sample_rate, cli.bits_per_sample)?;

    Ok(())
}
