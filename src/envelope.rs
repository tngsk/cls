use crate::tone::ToneParams;

pub struct Envelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
}

impl Envelope {
    pub fn new(params: &ToneParams) -> Self {
        Self {
            attack: params.attack,
            decay: params.decay,
            sustain: params.sustain,
            release: params.release,
        }
    }

    pub fn get_amplitude(&self, time: f32, duration: f32) -> f32 {
        let note_off = duration - self.release;

        if time < self.attack {
            // Attack phase: linear ramp from 0 to 1
            time / self.attack
        } else if time < self.attack + self.decay {
            // Decay phase: linear ramp from 1 to sustain level
            let t = (time - self.attack) / self.decay;
            1.0 - (1.0 - self.sustain) * t
        } else if time < note_off {
            // Sustain phase: constant level
            self.sustain
        } else {
            // Release phase: linear ramp from sustain to 0
            let t = (time - note_off) / self.release;
            self.sustain * (1.0 - t)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tone::ToneParams;

    #[test]
    fn test_envelope_phases() {
        let params = ToneParams::default();
        let envelope = Envelope::new(&params);

        // Test attack phase
        assert!(envelope.get_amplitude(0.05, 1.0) < 0.5);

        // Test decay phase
        assert!(envelope.get_amplitude(0.15, 1.0) > 0.7);

        // Test sustain phase
        assert_eq!(envelope.get_amplitude(0.3, 1.0), 0.7);

        // Test release phase
        assert!(envelope.get_amplitude(0.95, 1.0) < 0.7);
    }
}
