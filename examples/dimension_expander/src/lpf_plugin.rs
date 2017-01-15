use vst2::plugin::{Category, Info, Plugin};
use vst2::buffer::AudioBuffer;

use biquad::BiQuad;
use constants::SAMPLERATE;

/// The Compressor
pub struct LPFPlugin {
    biquad_right: BiQuad,
    biquad_left: BiQuad,
    cutoff: f32,
    q: f32,
}

impl Default for LPFPlugin {
    fn default() -> LPFPlugin {
        LPFPlugin::new(6000.)
    }
}

impl LPFPlugin {
    fn new(cutoff: f32) -> LPFPlugin {
        let mut ret = LPFPlugin {
            biquad_left: BiQuad::new(),
            biquad_right: BiQuad::new(),
            cutoff: cutoff / (SAMPLERATE / 2.),
            q: 1.
        };

        ret.update_params();

        return ret;
    }

    fn update_params(&mut self) {
        let v = (self.cutoff * 0.3) * (SAMPLERATE / 2.);

        self.biquad_left.set_lpf2_params(v, self.q * 10.);
        self.biquad_right.set_lpf2_params(v, self.q * 10.);
    }
}

impl Plugin for LPFPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "LPFPlugin".to_string(),
            vendor: "slono".to_string(),
            unique_id: 243723072,
            version: 0001,
            inputs: 2,
            outputs: 2,
            parameters: 2,
            category: Category::Effect,

            ..Default::default()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.cutoff,
            1 => self.q,
            _ => 0.0,
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.1}", self.cutoff * (SAMPLERATE / 2.)),
            1 => format!("{:.1}", self.q * 10.),
            _ => "".to_string()
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Cutoff hihihi",
            1 => "Resonance",
            _ => "",
        }.to_string()
    }

    fn set_parameter(&mut self, index: i32, val: f32) {
        match index {
            0 => {
                self.cutoff = val;
                self.update_params();
            },
            1 => {
                self.q = val;
                self.update_params();
            },
            _ => (),
        }
    }

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        let (inputs, mut outputs) = buffer.split();

        // Assume 2 channels
        if inputs.len() < 2 || outputs.len() < 2 {
            return;
        }

        // Iterate over inputs as (&f32, &f32)
        let stereo_in = match inputs.split_at(1) {
            (l, r) => l[0].iter().zip(r[0].iter())
        };

        // Iterate over outputs as (&mut f32, &mut f32)
        let stereo_out = match outputs.split_at_mut(1) {
            (l, r) => l[0].iter_mut().zip(r[0].iter_mut())
        };

        // Zip and process
        for ((left_in, right_in), (left_out, right_out)) in stereo_in.zip(stereo_out) {
            *left_out = self.biquad_left.process(*left_in);
            *right_out = self.biquad_right.process(*right_in);
        }
    }
}

