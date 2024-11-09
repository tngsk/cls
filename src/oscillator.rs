use crate::tone::ToneError;
use rand::Rng;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub enum WaveformType {
    Sine,
    Square,
    Triangle,
    Saw,
    Pulse,
    WhiteNoise,
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

    pub fn generate(&self, phase: f32) -> f32 {
        match self {
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
            WaveformType::WhiteNoise => rand::thread_rng().gen_range(-1.0..=1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waveform_from_str() {
        assert!(matches!(
            WaveformType::from_str("sine"),
            Ok(WaveformType::Sine)
        ));
        assert!(matches!(
            WaveformType::from_str("square"),
            Ok(WaveformType::Square)
        ));
        assert!(matches!(WaveformType::from_str("invalid"), Err(_)));
    }

    #[test]
    fn test_sine_generation() {
        let osc = WaveformType::Sine;
        assert!((osc.generate(0.0) - 0.0).abs() < 1e-6);
        assert!((osc.generate(0.25) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_square_generation() {
        let osc = WaveformType::Square;
        assert_eq!(osc.generate(0.25), 1.0);
        assert_eq!(osc.generate(0.75), -1.0);
    }
}
