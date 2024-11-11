use crate::tone::ToneError;
use rand::Rng;
use std::f32::consts::PI;

/// ルックアップテーブルのサイズ (2^14)
const LOOKUP_SIZE: usize = 16384;
/// ルックアップテーブルのインデックスマスク
const LOOKUP_MASK: usize = LOOKUP_SIZE - 1;
/// 位相スケール (2^32)
const PHASE_SCALE: f64 = 4294967296.0;

#[derive(Debug, Clone)]
pub struct Oscillator {
    waveform: WaveformType,
    phase: u32,
    phase_increment: u32,
    sample_rate: f32,
    lookup_table: Vec<f32>,
}

/// 倍音合成のためのパラメータ
#[derive(Debug, Clone)]
pub struct HarmonicParams {
    /// (倍音次数, 振幅) のペア
    harmonics: Vec<(f32, f32)>,
}

impl Default for HarmonicParams {
    fn default() -> Self {
        Self {
            harmonics: vec![
                (1.0, 1.0),  // 基本周波数
                (2.0, 0.5),  // 第2倍音
                (3.0, 0.33), // 第3倍音
                (5.0, 0.2),  // 第5倍音
                (7.0, 0.14), // 第7倍音
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub enum WaveformType {
    Sine,
    Square,
    Triangle,
    Saw,
    Pulse,
    WhiteNoise,
    Harmonic(HarmonicParams),
}

impl WaveformType {
    /// 波形の1サンプルを生成する
    fn generate_sample(&self, phase: f32) -> f32 {
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
            WaveformType::WhiteNoise => 0.0, // ルックアップテーブルでは使用しない
            WaveformType::Harmonic(params) => {
                let mut sum = 0.0;
                let mut normalize = 0.0;

                for &(harmonic, amplitude) in &params.harmonics {
                    sum += (phase * 2.0 * PI * harmonic).sin() * amplitude;
                    normalize += amplitude;
                }

                sum / normalize
            }
        }
    }

    pub fn from_str(s: &str) -> Result<Self, ToneError> {
        match s.to_lowercase().as_str() {
            "sin" | "sine" => Ok(WaveformType::Sine),
            "square" => Ok(WaveformType::Square),
            "triangle" => Ok(WaveformType::Triangle),
            "saw" => Ok(WaveformType::Saw),
            "pulse" => Ok(WaveformType::Pulse),
            "noise" | "whitenoise" => Ok(WaveformType::WhiteNoise),
            "harmonic" => Ok(WaveformType::Harmonic(HarmonicParams::default())),
            _ => Err(ToneError::Parse(format!("Unknown waveform: {}", s))),
        }
    }
}

impl Oscillator {
    pub fn new(waveform: WaveformType, sample_rate: f32) -> Self {
        let mut lookup_table = Vec::with_capacity(LOOKUP_SIZE);

        // ルックアップテーブルの生成
        for i in 0..LOOKUP_SIZE {
            let phase = i as f32 / LOOKUP_SIZE as f32;
            let value = waveform.generate_sample(phase);
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
                let index = (self.phase >> 18) as usize;
                let frac = (self.phase & 0x3FFFF) as f32 / 262144.0;

                let y0 = self.lookup_table[index & LOOKUP_MASK];
                let y1 = self.lookup_table[(index + 1) & LOOKUP_MASK];
                let output = y0 + frac * (y1 - y0);

                self.phase = self.phase.wrapping_add(self.phase_increment);

                output
            }
        }
    }
}
