use crate::tone::ToneError;
use rand::Rng;
use std::f32::consts::PI;

const LOOKUP_SIZE: usize = 16384;
const LOOKUP_MASK: usize = LOOKUP_SIZE - 1;
const PHASE_SCALE: f64 = 4294967296.0; // 2^32

#[derive(Debug, Clone)]
pub struct Oscillator {
    waveform: WaveformType,
    phase: u32,
    phase_increment: u32,
    sample_rate: f32,
    lookup_table: Vec<f32>,
}

#[derive(Debug, Clone)]
pub enum WaveformType {
    Sine,
    Square,
    Triangle,
    Saw,
    Pulse,
    WhiteNoise,
}

impl Oscillator {
    pub fn new(waveform: WaveformType, sample_rate: f32) -> Self {
        let mut lookup_table = Vec::with_capacity(LOOKUP_SIZE);

        // Generate lookup table
        for i in 0..LOOKUP_SIZE {
            let phase = i as f32 / LOOKUP_SIZE as f32;
            let value = match &waveform {
                WaveformType::Sine => (phase * 2.0 * PI).sin(),
                WaveformType::Square => {
                    if phase < 0.5 {
                        1.0
                    } else {
                        -1.0
                    }
                }
                WaveformType::Triangle => {
                    if phase < 0.5 {
                        4.0 * phase - 1.0
                    } else {
                        3.0 - 4.0 * phase
                    }
                }
                WaveformType::Saw => 2.0 * phase - 1.0,
                WaveformType::Pulse => {
                    if phase < 0.1 {
                        1.0
                    } else {
                        -1.0
                    }
                }
                WaveformType::WhiteNoise => 0.0, // Noise doesn't use lookup table
            };
            lookup_table.push(value);
        }

        Self {
            waveform,
            phase: 0,
            phase_increment: 0,
            sample_rate,
            lookup_table,
        }
    }

    pub fn from_str(waveform_str: &str, sample_rate: f32) -> Result<Self, ToneError> {
        let waveform = WaveformType::from_str(waveform_str)?;
        Ok(Self::new(waveform, sample_rate))
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.phase_increment = ((frequency as f64 * PHASE_SCALE / self.sample_rate as f64) as u32)
            .max(0)
            .min(u32::MAX);
    }

    pub fn generate(&mut self) -> f32 {
        match self.waveform {
            WaveformType::WhiteNoise => rand::thread_rng().gen_range(-1.0..=1.0),
            _ => {
                // Calculate table lookup index and fractional part
                let index = (self.phase >> 18) as usize;
                let frac = (self.phase & 0x3FFFF) as f32 / 262144.0; // 2^18

                // Linear interpolation
                let y0 = self.lookup_table[index & LOOKUP_MASK];
                let y1 = self.lookup_table[(index + 1) & LOOKUP_MASK];
                let output = y0 + frac * (y1 - y0);

                // Update phase
                self.phase = self.phase.wrapping_add(self.phase_increment);

                output
            }
        }
    }
}

impl WaveformType {
    pub fn from_str(s: &str) -> Result<Self, ToneError> {
        match s.to_lowercase().as_str() {
            "sin" | "sine" => Ok(WaveformType::Sine),
            "square" => Ok(WaveformType::Square),
            "triangle" => Ok(WaveformType::Triangle),
            "saw" => Ok(WaveformType::Saw),
            "pulse" => Ok(WaveformType::Pulse),
            "noise" | "whitenoise" => Ok(WaveformType::WhiteNoise),
            _ => Err(ToneError::Parse(format!("Unknown waveform: {}", s))),
        }
    }
}
