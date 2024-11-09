use serde::Deserialize;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ToneError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parameter parse error: {0}")]
    Parse(String),

    #[error("Invalid frequency: {0}")]
    InvalidFrequency(f32),

    #[error("Invalid duration: {0}")]
    InvalidDuration(f32),

    #[error("Invalid envelope parameter: {0}")]
    InvalidEnvelope(String),

    #[error("S-expression parse error: {0}")]
    SExprParse(#[from] serde_lexpr::parse::Error),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(default)]
pub struct ToneParams {
    pub freq: f32,
    pub dur: f32,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub waveform: String,
}

impl ToneParams {
    pub fn validate(&self) -> Result<(), ToneError> {
        if !(20.0..=20000.0).contains(&self.freq) {
            return Err(ToneError::InvalidFrequency(self.freq));
        }

        if self.dur <= 0.0 {
            return Err(ToneError::InvalidDuration(self.dur));
        }

        if self.attack < 0.0 || self.decay < 0.0 || self.release < 0.0 {
            return Err(ToneError::InvalidEnvelope(
                "Attack, decay, and release times must be non-negative".to_string(),
            ));
        }

        if !(0.0..=1.0).contains(&self.sustain) {
            return Err(ToneError::InvalidEnvelope(format!(
                "Sustain level must be between 0 and 1, got {}",
                self.sustain
            )));
        }

        let total_time = self.attack + self.decay + self.release;
        if total_time > self.dur {
            return Err(ToneError::InvalidEnvelope(
                "Total envelope time exceeds duration".to_string(),
            ));
        }

        Ok(())
    }
}

impl FromStr for ToneParams {
    type Err = ToneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_lexpr::from_str(s).map_err(|e| ToneError::Parse(e.to_string()))
    }
}

impl Default for ToneParams {
    fn default() -> Self {
        Self {
            freq: 440.0,  // A4 note
            dur: 1.0,     // 1 second
            attack: 0.1,  // 100ms
            decay: 0.1,   // 100ms
            sustain: 0.7, // 70% amplitude
            release: 0.1, // 100ms
            waveform: "sin".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_params() {
        let params = ToneParams::default();
        assert_eq!(params.freq, 440.0);
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_invalid_frequency() {
        let mut params = ToneParams::default();
        params.freq = 1.0;
        assert!(matches!(
            params.validate(),
            Err(ToneError::InvalidFrequency(_))
        ));
    }

    #[test]
    fn test_invalid_duration() {
        let mut params = ToneParams::default();
        params.dur = -1.0;
        assert!(matches!(
            params.validate(),
            Err(ToneError::InvalidDuration(_))
        ));
    }

    #[test]
    fn test_invalid_envelope() {
        let mut params = ToneParams::default();
        params.sustain = 1.5;
        assert!(matches!(
            params.validate(),
            Err(ToneError::InvalidEnvelope(_))
        ));
    }
}
