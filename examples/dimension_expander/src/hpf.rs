use constants::SAMPLERATE;

// High pass
pub struct HighPassFilter {
    a0: f32,
    a1: f32,
    z_1: f32,
}

impl HighPassFilter {
    pub fn new() -> HighPassFilter {
        let mut ret = HighPassFilter {
            a0: 0.,
            a1: 0.,
            z_1: 0.,
        };

        ret.set_frequency(6000.);

        ret
    }

    pub fn set_frequency(&mut self, freq: f32) {
        let v = freq / (SAMPLERATE / 2.0);
        println!("set frequency {}", v);
        self.a1 = v;
        self.a0 = self.a1 - 1.;
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        let ret: f32 = self.a0 * sample + self.a1 * self.z_1;
        self.z_1 = sample;

        ret
    }
}

